
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include "graph_c.h"

graph_t* create_graph() {
    // default allocation 25 squares (5x5), 100 edges
    graph_t* graph = calloc(1, sizeof(graph_t));
    if (graph == NULL) {
        return NULL;
    }
    graph->squares_len = INITIAL_SQUARES_LEN;
    graph->squares_width = INITIAL_SQUARES_WIDTH;

    /* Here's some not strictly portable code: we will go ahead and assume that 0'ing the
     * memory allocation will yield NULL pointers because in virtually every relevant 
     * hardware architecture that is true, despite that assumption being technically invalid
     * in the C standard
    */
    graph->squares = calloc(INITIAL_SQUARES_LEN, sizeof(internal_square_t));
    graph->edges = calloc(INITIAL_SQUARES_LEN, sizeof(internal_pathedge_t));
    return graph;
}

void destroy_graph(graph_t *self) {
    free(self->squares);
    free(self->edges);
    self = NULL;
}

void reallocate_squares(graph_t *self, uint32_t new_width) {
    uint32_t new_size = new_width * new_width;
    if (new_size > MAX_SQUARES_LEN) {
        // TODO: Error handling
        exit(-1);
    }
    internal_square_t* old_array = self->squares;
    self->squares = calloc(new_size, sizeof(internal_square_t));
    // memcpy each row of the old array into the new
    uint32_t rows = self->squares_len / self->squares_width;
    for (int i = 0; i < rows; i++) {
        memcpy(self->squares + (i * new_width), old_array + (i * self->squares_width), sizeof(internal_square_t) * self->squares_width);
    }
    self->squares_width = new_width;
    self->squares_len = new_size;
    // clean up old array
    free(old_array);

    // TODO: Now reallocate pathedges if they are indexed the same as squares
}

insert_result_t add_square(graph_t *self, square_t new_square) {
    uint32_t offset = new_square.x + (self->squares_width * new_square.y);
    // bounds checks, if it's beyond the bounds of the array, it will reallocate the memory for a bigger graph
    if (offset >= self->squares_len || new_square.x >= self->squares_width) {
        // TODO: Error handling
        uint32_t new_width = min(self->squares_width * 2, MAX_SQUARES_LEN);
        while (new_width < MAX_SQUARES_LEN) {
            // calculate what the new offset would be
            uint32_t offset = new_square.x + (new_width * new_square.y);
            // check if it's valid using the same logic as before
            if (offset >= min(new_width * new_width, MAX_SQUARES_LEN) || new_square.x >= new_width) {
                // still not wide enough, increase again
                new_width *= 2;
                continue;
            } else {
                reallocate_squares(self, new_width);
                break;
            }
            char msg[255];
            snprintf(msg, 255, "Could not allocate large enough array for x=%i", new_square.x);
            return (insert_result_t) { INSERT_RESULT_ERROR, msg};
        }
    }
    internal_square_t* position = self->squares + offset;
    *position = (internal_square_t) { NORMAL_SQUARE, new_square.x, new_square.y };
    return (insert_result_t) { INSERT_RESULT_SUCCESS, "" };
}

/**
 * Inserts a new pathedge into the internal_pathedge object
 * Assumes that there is space and does nothing otherwise
 * a little function for readability and convenience
*/
insert_result_t add_new_end_square(internal_pathedge_t *ipath, internal_square_t new_square) {
    if (ipath->end_square1.square_type == EMPTY_SQUARE) {
        ipath->end_square1 = (internal_square_t) { NORMAL_SQUARE, new_square.x, new_square.y };
        return (insert_result_t) { INSERT_RESULT_SUCCESS, "" };
    }

    if (ipath->end_square2.square_type == EMPTY_SQUARE) {
        ipath->end_square2 = (internal_square_t) { NORMAL_SQUARE, new_square.x, new_square.y };
        return (insert_result_t) { INSERT_RESULT_SUCCESS, "" };
    }

    if (ipath->end_square3.square_type == EMPTY_SQUARE) {
        ipath->end_square3 = (internal_square_t) { NORMAL_SQUARE, new_square.x, new_square.y };
        return (insert_result_t) { INSERT_RESULT_SUCCESS, "" };
    }

    if (ipath->end_square4.square_type == EMPTY_SQUARE) {
        ipath->end_square4 = (internal_square_t) { NORMAL_SQUARE, new_square.x, new_square.y };
        // in this case we also allocate new space and update link
        // TODO: don't write this next line of code until I figure out the dealloc, as a matter of writing alloc/free at the same tiem
        //ipath->next = calloc(1, sizeof(internal_pathedge_t));
        // return (insert_result_t) { INSERT_RESULT_SUCCESS, "" };
    }
    return (insert_result_t) { INSERT_RESULT_WARNING, "" };
}

insert_result_t add_pathedge(graph_t *self, pathedge_t new_edge) {
    uint32_t offset_src = new_edge.start_square.x + (self->squares_width * new_edge.start_square.y);
    uint32_t offset_dest = new_edge.end_square.x + (self->squares_width * new_edge.end_square.y);
    // check that both sides are already inserted into the graph
    internal_square_t start = self->squares[offset_src];
    internal_square_t stop = self->squares[offset_dest];
    if (start.square_type == EMPTY_SQUARE) {
        insert_result_t err_result = (insert_result_t) { INSERT_RESULT_ERROR, "" };
        snprintf(err_result.message, 140, "Start square (%i, %i) was empty", new_edge.start_square.x, new_edge.start_square.y);
        return err_result;
    }
    if (stop.square_type == EMPTY_SQUARE) {
        insert_result_t err_result = (insert_result_t) { INSERT_RESULT_ERROR, "" };
        snprintf(err_result.message, 140, "End square (%i, %i) was empty", new_edge.end_square.x, new_edge.end_square.y);
        return err_result;
    }

    // next find the pathedge linked list for an available space to put our new edge
    internal_pathedge_t *next_empty = (self->edges + offset_src);
    while (next_empty->next != NULL) {
        next_empty = next_empty->next;
    }

    // we found a pathedge without a next link, which means that it has space
    insert_result_t result = add_new_end_square(next_empty, stop);
    if (result.status != INSERT_RESULT_SUCCESS) {
        insert_result_t err_result = (insert_result_t) { INSERT_RESULT_ERROR, "" };
        snprintf(err_result.message, 140, "Pathedge (%i, %i) -> (%i, %i) could not be inserted!", start.x, start.y, stop.x, stop.y);
        return err_result;
    }
    return result;
}


uint32_t count_squares(graph_t *self) {
    uint32_t count = 0;
    for (uint32_t i = 0; i < self->squares_len; i++) {
        if (self->squares[i].square_type == NORMAL_SQUARE) {
            count += 1;
        }
    }
    return count;
}

square_t get_empty_square() {
    return (square_t) { 0, 0 };
}

pathedge_t get_empty_pathedge() {
    return (pathedge_t) { get_empty_square(), get_empty_square() };
}

square_t convert_to_square(internal_square_t square) {
    return (square_t) { square.x, square.y };
}

find_result_t get_square(graph_t* self, uint32_t x, uint32_t y) {
    uint32_t offset = x + self->squares_width * y;
    internal_square_t found = *(self->squares + offset);

    if (found.square_type != EMPTY_SQUARE) {
        return (find_result_t) { FIND_RESULT_RETURN_SQUARE, convert_to_square(found), get_empty_pathedge() };
    }

    return (find_result_t) {
        FIND_RESULT_EMPTY,
        get_empty_square(),
        get_empty_pathedge()
    };
}

bool square_eq(square_t first, square_t compare) {
    return (first.x == compare.x && first.y == compare.y);
}

find_result_t get_pathedge(graph_t *self, square_t start, square_t end) {
    uint32_t offset = start.x + self->squares_width * start.y;
    internal_pathedge_t* next = (self->edges + offset);
    internal_square_t found = (internal_square_t) { EMPTY_SQUARE, 0, 0 };
    while (next != NULL && found.square_type == EMPTY_SQUARE) {
        if (next->end_square1.square_type == NORMAL_SQUARE && square_eq(convert_to_square(next->end_square1), end)) {
            found = next->end_square1;
        } else if (next->end_square2.square_type == NORMAL_SQUARE && square_eq(convert_to_square(next->end_square2), end)) {
            found = next->end_square2;
        } else if (next->end_square3.square_type == NORMAL_SQUARE && square_eq(convert_to_square(next->end_square3), end)) {
            found = next->end_square3;
        } else if (next->end_square4.square_type == NORMAL_SQUARE && square_eq(convert_to_square(next->end_square4), end)) {
            found = next->end_square4;
        } else {
            next = next->next;
        }
    }
    if (found.square_type != EMPTY_SQUARE) {
        return (find_result_t) { FIND_RESULT_RETURN_PATHEDGE, get_empty_square(), (pathedge_t) { start, convert_to_square(found) } };
    }

    return (find_result_t) { FIND_RESULT_EMPTY, get_empty_square(), get_empty_pathedge() };
}