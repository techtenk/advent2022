use cty::{uint32_t, c_void as graph_t};
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct square_t {
    pub x: uint32_t,
    pub y: uint32_t
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct pathedge_t {
    pub start_square: square_t,
    pub end_square: square_t
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(unused)]
pub enum FindResultStatus {
    FindResultEmpty = 1, // item was not found
    FindResultError = 2, // error other than just not found
    FindResultReturnSquare = 4,
    FindResultReturnPathedge = 8
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct find_result_t {
    pub status: FindResultStatus,
    pub square: square_t,
    pub pathedge: pathedge_t
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(unused)]
pub enum InsertResultStatus {
    Success,
    Warning,
    Error
}

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(unused)]
pub struct insert_result_t {
    status: InsertResultStatus,
    message: [cty::c_char; 140] // error messages capped at a tweet so that we don't have to allocate
}

#[allow(unused)]
extern "C" {
    pub fn create_graph() -> *const graph_t;
    pub fn destroy_graph(graph: *const graph_t);
    pub fn add_square(graph: *const graph_t, new_square: square_t) -> insert_result_t;
    pub fn get_square(graph: *const graph_t, x: uint32_t, y: uint32_t) -> find_result_t;
    pub fn add_pathedge(graph: *const graph_t, new_edge: pathedge_t) -> insert_result_t;
    pub fn get_pathedge(graph: *const graph_t, start: square_t, end: square_t) -> find_result_t;
    pub fn count_squares(graph: *const graph_t) -> uint32_t;
}