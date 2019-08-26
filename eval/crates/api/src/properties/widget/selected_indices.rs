use std::collections::HashSet;

use crate::prelude::*;

property!(
    /// `SelectedIndices` describes a list of selected indices.
    SelectedIndices(HashSet<usize>)
);
