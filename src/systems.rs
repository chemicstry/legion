//! Automatic query scheduling and parallel execution.

pub use crate::internals::systems::{
    command::{CommandBuffer, WorldWritable},
    resources::{Fetch, FetchMut, Resource, ResourceSet, ResourceTypeId, Resources},
    schedule::{Builder, Executor, Runnable, Schedulable, Schedule, Step},
    system::{QuerySet, System, SystemAccess, SystemBuilder, SystemFn, SystemId},
};
