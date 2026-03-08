pub mod lifecycle;
pub mod part;
pub mod change;
pub mod errors;
pub mod repo;

pub use lifecycle::Lifecycle;
pub use part::Part;
pub use change::Change;
pub use errors::PlmError;
pub use repo::{Repo, RepoSnapshot};

use chrono::Utc;

pub fn app_create_part(repo: &impl Repo, number: String) -> Result<(), PlmError> {
    let mut snap = repo.load()?;

    if snap.parts.iter().any(|p| p.number == number) {
        return Err(PlmError::AlreadyExists(number));
    }

    let p = Part::new(number);
    snap.parts.push(p);

    repo.save(&snap)?;
    Ok(())
}

pub fn app_promote_part(
    repo: &impl Repo,
    number: String,
    to: Lifecycle,
    eco: String,
    reason: String,
) -> Result<(), PlmError> {
    let mut snap = repo.load()?;

    let part = snap
        .parts
        .iter_mut()
        .find(|p| p.number == number)
        .ok_or_else(|| PlmError::NotFound(number.clone()))?;

    // This calls the domain entity's promote method while also propagating any promotion errors
    // so the operation stops before making further state changes.
    part.promote(to.clone())?;

    // Audit record
    snap.changes.push(Change {
        part_number: number,
        eco,
        from: part.last_from.clone().expect("set in promote"),
        to,
        reason,
        at_utc: Utc::now(),
    });

    repo.save(&snap)?;
    Ok(())
}

pub fn app_change_order(
    repo: &impl Repo,
    number: String,
    eco: String,
    reason: String,
) -> Result<(), PlmError> {
    let mut snap = repo.load()?;

    let part = snap
        .parts
        .iter()
        .find(|p| p.number == number)
        .ok_or_else(|| PlmError::NotFound(number.clone()))?;

    let lifecycle = part.lifecycle.clone();

    snap.changes.push(Change {
        part_number: number,
        eco,
        from: lifecycle.clone(),
        to: lifecycle,
        reason,
        at_utc: Utc::now(),
    });

    repo.save(&snap)?;
    Ok(())
}

pub fn app_get_part(repo: &impl Repo, number: String) -> Result<Part, PlmError> {
    let snap = repo.load()?;
    snap.parts
        .into_iter()
        .find(|p| p.number == number)
        .ok_or_else(|| PlmError::NotFound(number))
}

pub fn app_list_parts(repo: &impl Repo) -> Result<Vec<Part>, PlmError> {
    let snap = repo.load()?;
    Ok(snap.parts)
}

pub fn app_history(repo: &impl Repo, number: String) -> Result<Vec<Change>, PlmError> {
    let snap = repo.load()?;
    Ok(snap
        .changes
        .into_iter()
        .filter(|c| c.part_number == number)
        .collect())
}

