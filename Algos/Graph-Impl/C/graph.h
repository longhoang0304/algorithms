#ifndef GRAPH_H
#define GRAPH_H

#include "vertex.h"

typedef struct G {
    int vertexCount;
    Vertex **vertecies;
} Graph;

Graph* NewGraph();
Graph* NewGraphWithParams(int vertexCount, Vertex **vertecies);
Graph* AddVertex(Graph* g, Vertex *v);

#endif /* GRAPH_H */
