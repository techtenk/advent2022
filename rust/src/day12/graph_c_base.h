#include <stdint.h>

#ifndef GRAPH_C_BASE
#define GRAPH_C_BASE

/**
 * square_t will be our vertex
 * squares are index with x and y
*/
typedef struct square_s {
    uint32_t x;
    uint32_t y;
} square_t;

/**
 * pathedges keep a copy of the square, not a reference. To find the square in the graph
 * we will use the x,y because that's how squares are indexed. pathedges are directional,
 * so we will need to add one for each direction
 * 
*/
typedef struct pathedge_s {
    square_t start_square;
    square_t end_square;
} pathedge_t;

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

typedef struct graph_s {
    uint32_t squares_len;
    uint32_t squares_width;
    internal_square_t *squares;
    internal_pathedge_t *edges;
} graph_t;

#endif