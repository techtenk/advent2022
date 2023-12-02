#include "graph_c_base.h"

#ifndef GRAPH_C_OPTIONS
#define GRAPH_C_OPTIONS

enum FIND_RESULT_STATUS {
    FIND_RESULT_EMPTY = 1, // item was not found
    FIND_RESULT_ERROR = 2, // error other than just not found
    FIND_RESULT_RETURN_SQUARE = 4,
    FIND_RESULT_RETURN_PATHEDGE = 8
};

typedef struct find_result_s {
    enum FIND_RESULT_STATUS status;
    square_t square;
    pathedge_t pathedge;
} find_result_t;

enum INSERT_RESULT_STATUS {
    INSERT_RESULT_SUCCESS,
    INSERT_RESULT_WARNING,
    INSERT_RESULT_ERROR
};

typedef struct insert_result_s {
    enum INSERT_RESULT_STATUS status;
    char message[140]; // error messages capped at a tweet so that we don't have to allocate
} insert_result_t;

#endif