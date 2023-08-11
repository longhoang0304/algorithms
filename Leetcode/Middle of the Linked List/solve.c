/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */


typedef struct ListNode* ListNodePtr;
ListNodePtr middleNode(ListNodePtr head){
    ListNodePtr start = head;
    ListNodePtr end = head;
    int counter = 1;
    while (end) {
        end = end->next;
        counter += 1;
        if (counter % 2) start = start->next;
    }
    return start;
}