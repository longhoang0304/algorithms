#ifndef VERTEX_H
#define VERTEX_H

typedef struct V {
    Vertex *parent;
    Vertex **children;
    int value;
    int childCount;
} Vertex;

Vertex* NewVertex();
Vertex* NewVertexWithValue(int value);
Vertex* NewVertexWithParent(Vertex *parent);
Vertex* NewVertexWithParentAndValue(Vertex *parent, int value);
Vertex* AddChild(Vertex *v, Vertex *child);

#endif /* VERTEX_H */
