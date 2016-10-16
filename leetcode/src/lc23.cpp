// Merge k sorted linked lists and return it as one sorted list.
// Analyze and describe its complexity.

// Definition for singly-linked list.
#include <algorithm>
#include <vector>
#include <iostream>



using namespace std;



struct ListNode {
  int val;
  ListNode *next;
  ListNode(int x) : val(x), next(NULL) {}
};

class Solution {
public:
  ListNode* mergeKLists(vector<ListNode*>& lists) {
    auto Compare =  [](const ListNode *a, const ListNode *b) -> bool {
      if (a == nullptr) {
        return true;
      }
      if (b == nullptr) {
        return false;
      }
      return a->val > b->val;
    };

    if (lists.empty()) {
      return nullptr;
    }

    ListNode *head = nullptr;
    ListNode *p = nullptr;

    make_heap(lists.begin(), lists.end(), Compare);

    while (lists.size() > 0) {
      if (lists[0] == nullptr) {
        return head;
      } else {
        pop_heap(lists.begin(), lists.end(), Compare);
        if (head == nullptr) {
          head = lists[lists.size()-1];
          p = head;
        } else {
          p->next = lists[lists.size()-1];
          p = p->next;
        }
        if (lists[lists.size()-1] == nullptr) {
          lists.pop_back();
        } else {
          lists[lists.size()-1] = lists[lists.size()-1]->next;
          push_heap(lists.begin(), lists.end(), Compare);
        }
      }

    }
    return nullptr;
  }


};


int main(int argc, char *argv[])
{
  auto head = new ListNode(0);
  auto tail = new ListNode(10);


  auto lists = vector<ListNode*> {nullptr};

  auto solution = Solution{};

  solution.mergeKLists(lists);

}
