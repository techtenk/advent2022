
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include "graph_c.h"

const uint32_t INITIAL_SQUARES_LEN = 25;
const uint32_t INITIAL_SQUARES_WIDTH = 5;
const uint32_t MAX_SQUARES_LEN = 1000000;

enum SQUARE_TYPE {
    EMPTY_SQUARE,
    NORMAL_SQUARE
};

enum PATHEDGE_TYPE {
    EMPTY_PATHEDGE,
    NORMAL_PATHEDGE
};

typedef struct internal_square_s {
    enum SQUARE_TYPE square_type;
    uint32_t x;
    uint32_t y;
} internal_square_t;

/**
 * internal_pathedge_t will be our edges
 * For early implementation, here are the assumptions for pathedges
 * specifically around the data structure and arrangement
 * 
 * - We will allocate room for 4 pathedges per square by default
 * - pathedges will be indexed the same way as nodes with the source node being the key
 * - the internal_pathedge_t type will have an optional link (ptr) to next to allow for more than 4 edges
*/
typedef struct internal_pathedge_s {
    enum PATHEDGE_TYPE pathedge_type;
    /* start_square is implied by where this pathedge is indexed */
    internal_square_t end_square1;
    internal_square_t end_square2;
    internal_square_t end_square3;
    internal_square_t end_square4;
    struct internal_pathedge_s* next;
} internal_pathedge_t;

/** 
 * graph struct to contain the references to the vertices and edges
*/
struct graph_s {
    uint32_t squares_len;
    uint32_t squares_width;
    internal_square_t *squares;
    internal_pathedge_t *edges;
};

graph_t* create_graph() {
    // default allocation 25 squares (5x5), 100 edges
    graph_t *graph = calloc(1, sizeof(graph_t));
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

void add_square(graph_t *self, square_t new_square) {
    uint32_t offset = new_square.x + (self->squares_width * new_square.y);
    // bounds checks, if it's beyond the bounds of the array, it will reallocate the memory for a bigger graph
    if (offset >= self->squares_len) {
        // TODO: Error handling
        reallocate_squares(self, self->squares_width * 2);
    }
    internal_square_t* position = self->squares + offset;
    *position = (internal_square_t) { NORMAL_SQUARE, new_square.x, new_square.y };
}

/**
 * Inserts a new pathedge into the internal_pathedge object
 * Assumes that there is space and does nothing otherwise
 * a little function for readability and convenience
*/
void add_new_end_square(internal_pathedge_t *ipath, internal_square_t new_square) {
    if (ipath->end_square1.square_type == EMPTY_SQUARE) {
        ipath->end_square1 = (internal_square_t) { NORMAL_SQUARE, new_square.x, new_square.y };
        return;
    }

    if (ipath->end_square2.square_type == EMPTY_SQUARE) {
        ipath->end_square2 = (internal_square_t) { NORMAL_SQUARE, new_square.x, new_square.y };
        return;
    }

    if (ipath->end_square3.square_type == EMPTY_SQUARE) {
        ipath->end_square3 = (internal_square_t) { NORMAL_SQUARE, new_square.x, new_square.y };
        return;
    }

    if (ipath->end_square4.square_type == EMPTY_SQUARE) {
        ipath->end_square4 = (internal_square_t) { NORMAL_SQUARE, new_square.x, new_square.y };
        // in this case we also allocate new space and update link
        ipath->next = calloc(1, sizeof(internal_pathedge_t));
        return;
    }
    // maybe future warning here if we want to indicate improper usage
}

void add_pathedge(graph_t *self, pathedge_t new_edge) {
    uint32_t offset_src = new_edge.start_square.x + (self->squares_width * new_edge.start_square.y);
    uint32_t offset_dest = new_edge.end_square.x + (self->squares_width * new_edge.end_square.y);
    // check that both sides are already inserted into the graph
    internal_square_t start = self->squares[offset_src];
    internal_square_t stop = self->squares[offset_dest];
    if (start.square_type == EMPTY_SQUARE || stop.square_type == EMPTY_SQUARE) {
        // TODO: error handling
        return;
    }

    // next find the pathedge linked list for an available space to put our new edge
    internal_pathedge_t *next_empty = (self->edges + offset_src);
    while (next_empty->next != NULL) {
        next_empty = next_empty->next;
    }

    // we found a pathedge without a next link, which means that it has space
    add_new_end_square(next_empty, stop);
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