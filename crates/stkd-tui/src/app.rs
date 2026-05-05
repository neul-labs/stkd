use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use std::collections::HashMap;
use stkd_core::Repository;
use stkd_provider_api::MergeRequest;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Stacks,
    Status,
    Actions,
}

#[derive(Debug, Clone)]
pub struct StackEntry {
    pub name: String,
    pub is_current: bool,
    pub depth: usize,
    pub mr_number: Option<u64>,
    pub mr_url: Option<String>,
    pub parent: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct WorkingTreeStatus {
    pub clean: bool,
    pub staged: usize,
    pub modified: usize,
    pub untracked: usize,
}

#[derive(Debug, Clone)]
pub enum Modal {
    Confirm {
        title: String,
        message: String,
        on_confirm: ConfirmAction,
    },
    Input {
        title: String,
        value: String,
        on_submit: InputAction,
    },
    Progress {
        title: String,
        message: String,
    },
}

#[derive(Debug, Clone)]
pub enum ConfirmAction {
    DeleteBranch(String),
    LandBranch(String),
    LandStack,
}

#[derive(Debug, Clone)]
pub enum InputAction {
    CreateBranch,
}

#[derive(Debug, Clone)]
pub struct Notification {
    pub message: String,
    pub is_error: bool,
    pub expires_at: std::time::Instant,
}

#[derive(Debug, Clone)]
pub enum ActionResult {
    Success(String),
    Error(String),
}

pub struct App {
    pub repo: Repository,
    pub current_tab: Tab,
    pub stacks: Vec<Vec<StackEntry>>,
    pub selected_stack: usize,
    pub selected_branch: usize,
    pub current_branch: String,
    pub trunk: String,
    pub mr_details: HashMap<u64, MergeRequest>,
    pub working_tree: WorkingTreeStatus,
    pub modal: Option<Modal>,
    pub notification: Option<Notification>,
    pub show_help: bool,
    pub spinner_active: bool,
    pub last_action_result: Option<ActionResult>,
    pub should_quit: bool,
    pub rx: Option<tokio::sync::mpsc::Receiver<AppMessage>>,
    pub tx: tokio::sync::mpsc::Sender<AppMessage>,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    MrFetched(u64, MergeRequest),
    MrFetchFailed(u64, String),
    ActionComplete(ActionResult),
    RefreshStacks,
}

impl App {
    pub fn new() -> Result<Self> {
        let repo = Repository::open(".")?;
        let current_branch = repo.current_branch()?.unwrap_or_default();
        let trunk = repo.trunk().to_string();

        let (tx, rx) = tokio::sync::mpsc::channel(32);

        let mut app = Self {
            repo,
            current_tab: Tab::Stacks,
            stacks: Vec::new(),
            selected_stack: 0,
            selected_branch: 0,
            current_branch,
            trunk,
            mr_details: HashMap::new(),
            working_tree: WorkingTreeStatus::default(),
            modal: None,
            notification: None,
            show_help: false,
            spinner_active: false,
            last_action_result: None,
            should_quit: false,
            rx: Some(rx),
            tx,
        };

        app.refresh_stacks()?;
        app.refresh_working_tree()?;

        Ok(app)
    }

    pub fn refresh_stacks(&mut self) -> Result<()> {
        let graph = self.repo.load_graph()?;
        let branches = self.repo.storage().list_branches()?;

        let mut stacks: Vec<Vec<StackEntry>> = Vec::new();

        let mut roots: Vec<String> = branches
            .iter()
            .filter(|b| graph.parent(&b.name) == Some(&self.trunk))
            .map(|b| b.name.clone())
            .collect();
        roots.sort();

        for root in roots {
            let stack_branches = graph.stack(&root);
            let entries: Vec<StackEntry> = stack_branches
                .iter()
                .enumerate()
                .map(|(i, name)| {
                    let info = self.repo.storage().load_branch(name).ok().flatten();
                    StackEntry {
                        name: name.to_string(),
                        is_current: name == &self.current_branch,
                        depth: i,
                        mr_number: info.as_ref().and_then(|i| i.merge_request_id),
                        mr_url: info.as_ref().and_then(|i| i.merge_request_url.clone()),
                        parent: if i == 0 {
                            Some(self.trunk.clone())
                        } else {
                            Some(stack_branches[i - 1].to_string())
                        },
                    }
                })
                .collect();
            if !entries.is_empty() {
                stacks.push(entries);
            }
        }

        self.stacks = stacks;

        if self.selected_stack >= self.stacks.len() && !self.stacks.is_empty() {
            self.selected_stack = self.stacks.len() - 1;
        }
        if let Some(stack) = self.stacks.get(self.selected_stack) {
            if self.selected_branch >= stack.len() && !stack.is_empty() {
                self.selected_branch = stack.len() - 1;
            }
        }

        Ok(())
    }

    pub fn refresh_working_tree(&mut self) -> Result<()> {
        let statuses = self.repo.git().statuses(None)?;

        if statuses.is_empty() {
            self.working_tree = WorkingTreeStatus {
                clean: true,
                staged: 0,
                modified: 0,
                untracked: 0,
            };
            return Ok(());
        }

        let mut staged = 0;
        let mut modified = 0;
        let mut untracked = 0;

        for entry in statuses.iter() {
            let status = entry.status();
            if status.is_index_new() || status.is_index_modified() || status.is_index_deleted() {
                staged += 1;
            }
            if status.is_wt_modified() || status.is_wt_deleted() {
                modified += 1;
            }
            if status.is_wt_new() {
                untracked += 1;
            }
        }

        self.working_tree = WorkingTreeStatus {
            clean: false,
            staged,
            modified,
            untracked,
        };

        Ok(())
    }

    pub fn on_tick(&mut self) {
        // Collect all pending messages first to avoid borrow issues
        let mut messages = Vec::new();
        if let Some(rx) = &mut self.rx {
            while let Ok(msg) = rx.try_recv() {
                messages.push(msg);
            }
        }

        for msg in messages {
            match msg {
                AppMessage::MrFetched(id, mr) => {
                    self.mr_details.insert(id, mr);
                    self.spinner_active = false;
                }
                AppMessage::MrFetchFailed(id, err) => {
                    self.show_notification(format!("Failed to fetch MR #{}: {}", id, err), true);
                    self.spinner_active = false;
                }
                AppMessage::ActionComplete(result) => {
                    self.spinner_active = false;
                    self.last_action_result = Some(result.clone());
                    match result {
                        ActionResult::Success(msg) => self.show_notification(msg, false),
                        ActionResult::Error(msg) => self.show_notification(msg, true),
                    }
                    let _ = self.refresh_stacks();
                    let _ = self.refresh_working_tree();
                }
                AppMessage::RefreshStacks => {
                    let _ = self.refresh_stacks();
                    let _ = self.refresh_working_tree();
                }
            }
        }

        if let Some(ref notif) = self.notification {
            if std::time::Instant::now() >= notif.expires_at {
                self.notification = None;
            }
        }
    }

    pub async fn handle_event(&mut self, event: Event) -> Result<bool> {
        if self.modal.is_some() {
            return self.handle_modal_event(event).await;
        }

        if self.show_help {
            if let Event::Key(key) = event {
                if matches!(key.code, KeyCode::Char('?') | KeyCode::Esc | KeyCode::Char('q')) {
                    self.show_help = false;
                }
            }
            return Ok(false);
        }

        match event {
            Event::Key(key) => self.handle_key_event(key).await,
            _ => Ok(false),
        }
    }

    async fn handle_key_event(&mut self, key: KeyEvent) -> Result<bool> {
        match key.code {
            KeyCode::Char('q') if key.modifiers == KeyModifiers::NONE => {
                return Ok(true);
            }
            KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => {
                return Ok(true);
            }
            KeyCode::Tab => {
                self.next_tab();
            }
            KeyCode::BackTab => {
                self.prev_tab();
            }
            KeyCode::Char('?') => {
                self.show_help = true;
            }
            KeyCode::Char('g') => {
                self.refresh_stacks()?;
                self.refresh_working_tree()?;
                self.fetch_provider_status().await;
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.next_branch();
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.prev_branch();
            }
            KeyCode::Right | KeyCode::Char('l') => {
                self.next_stack();
            }
            KeyCode::Left | KeyCode::Char('h') => {
                self.prev_stack();
            }
            KeyCode::Enter => {
                if let Some(branch) = self.selected_branch_name() {
                    self.checkout_branch(&branch)?;
                }
            }
            KeyCode::Char('c') => {
                self.modal = Some(Modal::Input {
                    title: "Create Branch".to_string(),
                    value: String::new(),
                    on_submit: InputAction::CreateBranch,
                });
            }
            KeyCode::Char('d') => {
                if let Some(branch) = self.selected_branch_name() {
                    if branch != self.trunk {
                        self.modal = Some(Modal::Confirm {
                            title: "Delete Branch".to_string(),
                            message: format!("Are you sure you want to delete '{}'?", branch),
                            on_confirm: ConfirmAction::DeleteBranch(branch),
                        });
                    }
                }
            }
            KeyCode::Char('s') => {
                self.run_submit().await?;
            }
            KeyCode::Char('y') => {
                self.run_sync().await?;
            }
            KeyCode::Char('r') => {
                self.run_restack().await?;
            }
            KeyCode::Char('u') => {
                if let Ok(branch) = self.repo.up(1) {
                    self.current_branch = branch;
                    self.refresh_stacks()?;
                    self.show_notification(format!("Moved up to '{}'", self.current_branch), false);
                }
            }
            KeyCode::Char('n') => {
                if let Ok(branch) = self.repo.down(1) {
                    self.current_branch = branch;
                    self.refresh_stacks()?;
                    self.show_notification(format!("Moved down to '{}'", self.current_branch), false);
                }
            }
            KeyCode::Char('t') => {
                if let Ok(branch) = self.repo.top() {
                    self.current_branch = branch;
                    self.refresh_stacks()?;
                    self.show_notification(format!("Moved to top '{}'", self.current_branch), false);
                }
            }
            KeyCode::Char('b') => {
                if let Ok(branch) = self.repo.bottom() {
                    self.current_branch = branch;
                    self.refresh_stacks()?;
                    self.show_notification(format!("Moved to bottom '{}'", self.current_branch), false);
                }
            }
            _ => {}
        }

        Ok(false)
    }

    async fn handle_modal_event(&mut self, event: Event) -> Result<bool> {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Esc => {
                    self.modal = None;
                }
                _ => {
                    if let Some(modal) = self.modal.take() {
                        match modal {
                            Modal::Input { title, mut value, on_submit } => {
                                match key.code {
                                    KeyCode::Enter => {
                                        match on_submit {
                                            InputAction::CreateBranch => {
                                                if !value.is_empty() {
                                                    self.create_branch(&value).await?;
                                                }
                                            }
                                        }
                                    }
                                    KeyCode::Backspace => {
                                        value.pop();
                                        self.modal = Some(Modal::Input { title, value, on_submit });
                                    }
                                    KeyCode::Char(c) => {
                                        value.push(c);
                                        self.modal = Some(Modal::Input { title, value, on_submit });
                                    }
                                    _ => {
                                        self.modal = Some(Modal::Input { title, value, on_submit });
                                    }
                                }
                            }
                            Modal::Confirm { title, message, on_confirm } => {
                                match key.code {
                                    KeyCode::Char('y') | KeyCode::Enter => {
                                        match on_confirm {
                                            ConfirmAction::DeleteBranch(branch) => {
                                                self.delete_branch(&branch).await?;
                                            }
                                            ConfirmAction::LandBranch(branch) => {
                                                self.land_branch(&branch).await?;
                                            }
                                            ConfirmAction::LandStack => {
                                                self.land_stack().await?;
                                            }
                                        }
                                    }
                                    _ => {
                                        self.modal = Some(Modal::Confirm { title, message, on_confirm });
                                    }
                                }
                            }
                            Modal::Progress { title, message } => {
                                self.modal = Some(Modal::Progress { title, message });
                            }
                        }
                    }
                }
            }
        }
        Ok(false)
    }

    fn next_tab(&mut self) {
        self.current_tab = match self.current_tab {
            Tab::Stacks => Tab::Status,
            Tab::Status => Tab::Actions,
            Tab::Actions => Tab::Stacks,
        };
    }

    fn prev_tab(&mut self) {
        self.current_tab = match self.current_tab {
            Tab::Stacks => Tab::Actions,
            Tab::Status => Tab::Stacks,
            Tab::Actions => Tab::Status,
        };
    }

    fn next_branch(&mut self) {
        if let Some(stack) = self.stacks.get(self.selected_stack) {
            if self.selected_branch + 1 < stack.len() {
                self.selected_branch += 1;
            }
        }
    }

    fn prev_branch(&mut self) {
        if self.selected_branch > 0 {
            self.selected_branch -= 1;
        }
    }

    fn next_stack(&mut self) {
        if self.selected_stack + 1 < self.stacks.len() {
            self.selected_stack += 1;
            self.selected_branch = 0;
        }
    }

    fn prev_stack(&mut self) {
        if self.selected_stack > 0 {
            self.selected_stack -= 1;
            self.selected_branch = 0;
        }
    }

    pub fn selected_branch_name(&self) -> Option<String> {
        self.stacks
            .get(self.selected_stack)
            .and_then(|s| s.get(self.selected_branch))
            .map(|e| e.name.clone())
    }

    fn checkout_branch(&mut self, branch: &str) -> Result<()> {
        self.repo.checkout(branch)?;
        self.current_branch = branch.to_string();
        self.refresh_stacks()?;
        self.refresh_working_tree()?;
        self.show_notification(format!("Checked out '{}'", branch), false);
        Ok(())
    }

    async fn create_branch(&mut self, name: &str) -> Result<()> {
        self.repo.ensure_clean()?;
        self.repo.create_branch(name)?;
        self.current_branch = name.to_string();
        self.refresh_stacks()?;
        self.show_notification(format!("Created branch '{}'", name), false);
        Ok(())
    }

    async fn delete_branch(&mut self, branch: &str) -> Result<()> {
        self.repo.delete_branch(branch, false)?;
        self.refresh_stacks()?;
        self.show_notification(format!("Deleted branch '{}'", branch), false);
        Ok(())
    }

    async fn run_submit(&mut self) -> Result<()> {
        self.spinner_active = true;
        let tx = self.tx.clone();

        tokio::task::spawn_blocking(move || {
            let rt = tokio::runtime::Handle::current();
            let repo = match Repository::open(".") {
                Ok(r) => r,
                Err(e) => {
                    let _ = tx.blocking_send(AppMessage::ActionComplete(ActionResult::Error(format!("{:#}", e))));
                    return;
                }
            };

            let ctx = match rt.block_on(stkd_engine::ProviderContext::from_repo(&repo)) {
                Ok(c) => c,
                Err(e) => {
                    let _ = tx.blocking_send(AppMessage::ActionComplete(ActionResult::Error(format!("Provider error: {:#}", e))));
                    return;
                }
            };

            let opts = stkd_engine::SubmitOptions::default();
            match rt.block_on(stkd_engine::submit(&repo, opts, ctx.provider(), &ctx.repo_id)) {
                Ok(result) => {
                    let msg = format!(
                        "Submitted {} MRs, updated {} MRs",
                        result.created.len(),
                        result.updated.len()
                    );
                    let _ = tx.blocking_send(AppMessage::ActionComplete(ActionResult::Success(msg)));
                }
                Err(e) => {
                    let _ = tx.blocking_send(AppMessage::ActionComplete(ActionResult::Error(format!("{:#}", e))));
                }
            }
        });

        Ok(())
    }

    async fn run_sync(&mut self) -> Result<()> {
        self.spinner_active = true;
        let tx = self.tx.clone();

        tokio::task::spawn_blocking(move || {
            let rt = tokio::runtime::Handle::current();
            let repo = match Repository::open(".") {
                Ok(r) => r,
                Err(e) => {
                    let _ = tx.blocking_send(AppMessage::ActionComplete(ActionResult::Error(format!("{:#}", e))));
                    return;
                }
            };

            let provider = match rt.block_on(stkd_engine::ProviderContext::from_repo(&repo)) {
                Ok(ctx) => Some(ctx),
                Err(_) => None,
            };

            let opts = stkd_engine::SyncOptions::default();
            let result = if let Some(ref ctx) = provider {
                rt.block_on(stkd_engine::sync(&repo, opts, Some(ctx.provider()), Some(&ctx.repo_id)))
            } else {
                rt.block_on(stkd_engine::sync(&repo, opts, None, None))
            };

            match result {
                Ok(result) => {
                    let msg = format!(
                        "Synced: {} deleted, {} restacked",
                        result.deleted_branches.len(),
                        result.restacked.len()
                    );
                    let _ = tx.blocking_send(AppMessage::ActionComplete(ActionResult::Success(msg)));
                }
                Err(e) => {
                    let _ = tx.blocking_send(AppMessage::ActionComplete(ActionResult::Error(format!("{:#}", e))));
                }
            }
        });

        Ok(())
    }

    async fn run_restack(&mut self) -> Result<()> {
        self.spinner_active = true;
        let tx = self.tx.clone();

        tokio::task::spawn_blocking(move || {
            let repo = match Repository::open(".") {
                Ok(r) => r,
                Err(e) => {
                    let _ = tx.blocking_send(AppMessage::ActionComplete(ActionResult::Error(format!("{:#}", e))));
                    return;
                }
            };

            let opts = stkd_engine::RestackOptions::default();
            match stkd_engine::restack(&repo, opts) {
                Ok(result) => {
                    let msg = format!("Restacked {} branches", result.restacked.len());
                    let _ = tx.blocking_send(AppMessage::ActionComplete(ActionResult::Success(msg)));
                }
                Err(e) => {
                    let _ = tx.blocking_send(AppMessage::ActionComplete(ActionResult::Error(format!("{:#}", e))));
                }
            }
        });

        Ok(())
    }

    async fn land_branch(&mut self, branch: &str) -> Result<()> {
        self.show_notification(format!("Land branch '{}' not yet implemented in TUI", branch), true);
        Ok(())
    }

    async fn land_stack(&mut self) -> Result<()> {
        self.show_notification("Land stack not yet implemented in TUI".to_string(), true);
        Ok(())
    }

    async fn fetch_provider_status(&mut self) {
        self.spinner_active = true;
        let tx = self.tx.clone();

        let mr_numbers: Vec<u64> = self
            .stacks
            .iter()
            .flat_map(|s| s.iter().filter_map(|e| e.mr_number))
            .collect();

        if mr_numbers.is_empty() {
            self.spinner_active = false;
            return;
        }

        tokio::task::spawn_blocking(move || {
            let rt = tokio::runtime::Handle::current();
            let repo = match Repository::open(".") {
                Ok(r) => r,
                Err(_) => return,
            };

            let ctx = match rt.block_on(stkd_engine::ProviderContext::from_repo(&repo)) {
                Ok(c) => c,
                Err(_) => return,
            };

            for mr_num in mr_numbers {
                match rt.block_on(ctx.provider().get_mr(&ctx.repo_id, mr_num.into())) {
                    Ok(mr) => {
                        let _ = rt.block_on(tx.send(AppMessage::MrFetched(mr_num, mr)));
                    }
                    Err(e) => {
                        let _ = rt.block_on(tx.send(AppMessage::MrFetchFailed(mr_num, format!("{:#}", e))));
                    }
                }
            }
        });
    }

    pub fn show_notification(&mut self, message: String, is_error: bool) {
        self.notification = Some(Notification {
            message,
            is_error,
            expires_at: std::time::Instant::now() + std::time::Duration::from_secs(5),
        });
    }

    pub fn selected_entry(&self) -> Option<&StackEntry> {
        self.stacks
            .get(self.selected_stack)
            .and_then(|s| s.get(self.selected_branch))
    }
}
