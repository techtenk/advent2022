#[cfg(test)]
mod tests {
    use cty::{c_void, uint32_t};

    use crate::day12::graph_c_bindings::*;

    use super::*;
    
    #[test]
    fn test_create_graph() {
        unsafe {
            let graph: *const c_void = create_graph();
            println!("Allocated the graph from unsafe block");
            assert!(!graph.is_null());
            destroy_graph(graph);
        }

    }

    #[test]
    fn test_add_squares() {
        unsafe {
            let graph: *const c_void = create_graph();
            add_square(graph, square_t { x: 0, y: 0 });
            assert_eq!(count_squares(graph), 1);
            destroy_graph(graph);
        }
    }

    #[test]
    fn test_add_squares_at_end() {
        unsafe {
            let graph: *const c_void = create_graph();
            add_square(graph, square_t { x: 4, y: 4 });
            assert_eq!(count_squares(graph), 1);
            destroy_graph(graph);
        }
    }

    /**
     * Tests if we try to add a square beyond the initial size of the graph, if it
     * reallocates correctly and succeeds
     */
    #[test]
    fn test_add_squares_trigger_reallocate() {
        unsafe {
            let graph: *const c_void = create_graph();
            // add a square to each "row" to make sure they all get moved properly
            add_square(graph, square_t { x: 0, y: 0 });
            add_square(graph, square_t { x: 1, y: 1 });
            add_square(graph, square_t { x: 2, y: 2 });
            add_square(graph, square_t { x: 3, y: 3 });
            add_square(graph, square_t { x: 5, y: 4 });
            assert_eq!(count_squares(graph), 5);
            destroy_graph(graph);
        }
    }

    /*
     * Test if we try to add a square with a large x, if it
     * can choose an appropriate row width
     */
    #[test]
    fn test_add_large_x() {
        unsafe {
            let graph = create_graph();
            let _result = add_square(graph, square_t { x: 11, y: 0});
            // if the array didn't reallocate at all then the we would find it at (1, 2) which is offset 11
            let result_should_be_empty = get_square(graph, 1, 2);
            assert!(result_should_be_empty.status == FindResultStatus::FindResultEmpty);

            // if it reallocated "normally", it would expand the row width to 10 and then (1, 1) would be where it is
            let another_should_be_empty = get_square(graph, 1, 1);
            assert!(another_should_be_empty.status == FindResultStatus::FindResultEmpty);

            // good, now see if we can fetch it from the actual location
            let result_square = get_square(graph, 11, 0);
            assert!(result_square.status == FindResultStatus::FindResultReturnSquare);
        }
    
    }

    /*
     * Test if we try to add a square with a very large y, if it
     * can choose an appropriate row width. The intent is to choose a
     * y that is large enough that the grid can't be square anymore 
     * because of the MAX_SQUARES_LEN value
     * Ex: > 10000 if MAX_SQUARES_LEN is 1,000,000
     */
    fn test_add_large_y() {
        unsafe {
            let graph = create_graph();
            add_square(graph, square_t { x: 0, y: 12000});
        }
    }

    /**
     * Test the bounds of reallocation, to max len of 1,000,000
     */
    fn test_reallocation_max_len() {}

    /**
     * Test that it can allocate with width > 1000
     */
    fn test_reallocation_large_rows() {}

    /**
     * test for getting specific square
     */
    #[test]
    fn test_get_square() {
        unsafe {
            let graph: *const c_void = create_graph();
            add_square(graph, square_t { x: 4, y: 4 });
            
            let result: find_result_t = get_square(graph, 4, 4);
            assert!(result.status == FindResultStatus::FindResultReturnSquare);
            destroy_graph(graph);
        }
        
    }

    /**
     * test for getting specific pathedge
     */
    #[test]
    fn test_get_pathedge(){
        unsafe {
            let graph: *const c_void = create_graph();
            let start_square = square_t { x: 4, y: 4 };
            let end_square = square_t { x: 3, y: 4 };
            let another_start = square_t { x: 3, y: 3 };
            let another_end = square_t { x: 2, y: 2 };
            add_square(graph, start_square);
            add_square(graph, end_square);
            add_square(graph, another_start);
            add_square(graph, another_end);
            add_pathedge(graph, pathedge_t { start_square, end_square });
            add_pathedge(graph, pathedge_t { start_square: another_start, end_square: another_end });
            let result = get_pathedge(graph, start_square, end_square);
            assert_eq!(result.status, FindResultStatus::FindResultReturnPathedge);
            assert_eq!(result.pathedge.start_square, start_square);
            assert_eq!(result.pathedge.end_square, end_square);
            let another_result = get_pathedge(graph, another_start, another_end);
            assert_eq!(another_result.status, FindResultStatus::FindResultReturnPathedge);
            assert_eq!(another_result.pathedge.start_square, another_start);
            assert_eq!(another_result.pathedge.end_square, another_end);
        }
    }

    /**
     * Test for whether pathedges are reallocated correctly when squares are reallocated
     */
    #[test]
    fn test_reallocate_pathedges(){
        unsafe {
            let graph: *const c_void = create_graph();
            
            // fill the first 2 rows with squares, with an edge for each from row 2 to row 1
            // since we index pathedge based off their start square and the start squares for row 2
            // will have to move, we have to move the pathedge memory around too
            for i in 0..5 {
                let start = square_t { x: i, y: 1 };
                let end = square_t { x: i, y: 0 };
                let pathedge = pathedge_t { start_square: start, end_square: end };
                add_square(graph, start);
                add_square(graph, end);
                add_pathedge(graph, pathedge);
            }

            // now add a square that makes the memory expand and check that we can still find the pathedges
            let mut found_all: bool = true;
            add_square(graph, square_t { x: 6, y: 0 });
            for i in 0..5 {
                let start = square_t { x: i, y: 0 };
                let end = square_t { x: i, y: 1 };
                let result = get_pathedge(graph, start, end);
                if result.status != FindResultStatus::FindResultReturnPathedge {
                    found_all = false;
                }
                if result.pathedge.start_square != start || result.pathedge.end_square != end {
                    found_all = false;
                }
            }
            assert_eq!(found_all, true);
        }
    }

    /**
     * Test adding a new pathedage that has to allocate another pathedge
     */
    #[test]
    fn test_add_pathedge_allocate_new_memory(){
        unsafe {
            let graph: *const c_void = create_graph();
            // five pathedges from one source node will force it to allocate more memory
            let source = square_t { x: 1, y: 1 };
            let mut dest = Vec::new();
            for i in 0..5 {
                dest.push(square_t { x: 0, y: i });
            }
            for d in &dest {
                add_pathedge(graph, pathedge_t { start_square: source, end_square: *d });
            }
            // now see if we can find the last one
            let result = get_pathedge(graph, source, *(dest.last().unwrap()));
            assert_eq!(result.status, FindResultStatus::FindResultReturnPathedge);
            assert_eq!(result.pathedge.start_square, source);
            assert_eq!(result.pathedge.end_square, *dest.last().unwrap());
        }

    }

    
}