#include <stdlib.h>
#include "graph.h"
#include "vertex.h"

Graph* NewGraph() {
    Graph* g = (Graph *)malloc(sizeof(Graph));
    g->vertexCount = 0;
    g->vertecies = NULL;
}

Graph* NewGraphWithParams(int vertexCount, Vertex **vertecies) {
    Graph* g = (Graph *)malloc(sizeof(Graph));
    g->vertexCount = vertexCount;
    g->vertecies = vertecies;
}

Graph* AddVertex(Graph *g, Vertex *v) {
    g->vertexCount++;

    if (g->vertecies == NULL) {
        g->vertecies = (Vertex **)malloc(g->vertexCount * sizeof(Vertex *));
    } else {
        g->vertecies = (Vertex **)realloc(g->vertecies, g->vertexCount * sizeof(Vertex *));
    }

    if (g->vertecies == NULL) {
        return g;
    }

    g->vertecies[g->vertexCount - 1] = v;
    return g;
}
