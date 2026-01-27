//! Repository traits for data access.

mod branch;
mod membership;
mod merge_request;
mod organization;
mod repository;
mod session;
mod user;

pub use branch::BranchRepository;
pub use membership::MembershipRepository;
pub use merge_request::MergeRequestRepository;
pub use organization::OrganizationRepository;
pub use repository::RepositoryRepository;
pub use session::SessionRepository;
pub use user::UserRepository;
