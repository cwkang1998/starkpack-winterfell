// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use crate::RowMatrix;
use air::EvaluationFrame;
use math::FieldElement;
use utils::collections::Vec;

// TRACE LOW DEGREE EXTENSION
// ================================================================================================
/// TODO: add docs
pub struct TraceLde<E: FieldElement> {
    main_segment_lde: RowMatrix<E::BaseField>,
    aux_segment_ldes: Vec<RowMatrix<E>>,
    blowup: usize,
}

impl<E: FieldElement> TraceLde<E> {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Creates a new trace low-degree extension table from the provided main trace segment LDE.
    pub fn new(main_trace_lde: RowMatrix<E::BaseField>, blowup: usize) -> Self {
        Self {
            main_segment_lde: main_trace_lde,
            aux_segment_ldes: Vec::new(),
            blowup,
        }
    }

    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Adds the provided auxiliary segment LDE to this trace LDE.
    pub fn add_aux_segment(&mut self, aux_segment_lde: RowMatrix<E>) {
        assert_eq!(
            self.main_segment_lde.num_rows(),
            aux_segment_lde.num_rows(),
            "number of rows in auxiliary segment must be of the same as in the main segment"
        );
        self.aux_segment_ldes.push(aux_segment_lde);
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns number of columns in the main segment of the execution trace.
    pub fn main_trace_width(&self) -> usize {
        self.main_segment_lde.num_cols()
    }

    /// Returns number of columns in the auxiliary segments of the execution trace.
    pub fn aux_trace_width(&self) -> usize {
        self.aux_segment_ldes
            .iter()
            .fold(0, |s, m| s + m.num_cols())
    }

    /// Returns the number of rows in the execution trace.
    pub fn trace_len(&self) -> usize {
        self.main_segment_lde.num_rows()
    }

    /// Returns blowup factor which was used to extend original execution trace into trace LDE.
    pub fn blowup(&self) -> usize {
        self.blowup
    }

    /// Reads current and next rows from the main trace segment into the specified frame.
    pub fn read_main_trace_frame_into(
        &self,
        lde_step: usize,
        frame: &mut EvaluationFrame<E::BaseField>,
    ) {
        // at the end of the trace, next state wraps around and we read the first step again
        let next_lde_step = (lde_step + self.blowup()) % self.trace_len();

        // copy main trace segment values into the frame
        frame.set_data(
            self.main_segment_lde.row(lde_step).into(),
            self.main_segment_lde.row(next_lde_step).into(),
        );
    }

    /// Reads current and next rows from the auxiliary trace segment into the specified frame.
    ///
    /// # Panics
    /// This currently assumes that there is exactly one auxiliary trace segment, and will panic
    /// otherwise.
    pub fn read_aux_trace_frame_into(&self, lde_step: usize, frame: &mut EvaluationFrame<E>) {
        // at the end of the trace, next state wraps around and we read the first step again
        let next_lde_step = (lde_step + self.blowup()) % self.trace_len();

        // copy auxiliary trace segment values into the frame
        let segment = &self.aux_segment_ldes[0];
        frame.set_data(
            segment.row(lde_step).into(),
            segment.row(next_lde_step).into(),
        );
    }

    /// Returns a reference to [Matrix] representing the main trace segment.
    pub fn get_main_segment(&self) -> &RowMatrix<E::BaseField> {
        &self.main_segment_lde
    }

    /// Returns a reference to a [Matrix] representing an auxiliary trace segment at the specified
    /// index.
    pub fn get_aux_segment(&self, aux_segment_idx: usize) -> &RowMatrix<E> {
        &self.aux_segment_ldes[aux_segment_idx]
    }
}
