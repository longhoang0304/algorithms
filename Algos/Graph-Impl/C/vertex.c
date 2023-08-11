#include <stdlib.h>
#include "vertex.h"

Vertex* NewVertex() {
    Vertex* v = (Vertex *)malloc(sizeof(Vertex));
    v->parent = NULL;
    v->children = NULL;
    v->childCount = 0;
    v->value = 0;

    return v;
}

Vertex* NewVertexWithValue(int value) {
    Vertex* v = (Vertex *)malloc(sizeof(Vertex));
    v->parent = NULL;
    v->children = NULL;
    v->childCount = 0;
    v->value = value;

    return v;
}

Vertex* NewVertexWithParent(Vertex *parent) {
    Vertex* v = (Vertex *)malloc(sizeof(Vertex));
    v->parent = parent;
    v->children = NULL;
    v->childCount = 0;
    v->value = 0;

    return v;
}

Vertex* NewVertexWithParentAndValue(Vertex *parent, int value) {
    Vertex* v = (Vertex *)malloc(sizeof(Vertex));
    v->parent = parent;
    v->children = NULL;
    v->childCount = 0;
    v->value = value;

    return v;
}

Vertex* AddChild(Vertex *v, Vertex *child) {
    v->childCount += 1;
    if (v->children == NULL) {
        v->children = (Vertex **)malloc(sizeof(Vertex *));
    } else {
        v->children = (Vertex **)realloc(v->children, sizeof(Vertex *) * v->childCount);
    }

    v->children[v->childCount - 1] = child;
    child->parent = v;
}
