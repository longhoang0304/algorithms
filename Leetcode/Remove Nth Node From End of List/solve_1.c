/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */


typedef struct ListNode* ListNodePtr;
ListNodePtr removeNthFromEnd(ListNodePtr head, int n){
    ListNodePtr start = head;
    ListNodePtr end = head;
    ListNodePtr remove = head;
    // start -> remove -> end
    while(--n && end->next) end = end->next;
    while(end->next) {
        end = end->next;
        if (remove->next) start = remove, remove = remove->next;
    }
    if (remove == end && start == remove) return NULL;
    if (remove == end) start->next = NULL;
    if (remove == start) return start->next;
    else start->next = remove->next;
    return head;
}