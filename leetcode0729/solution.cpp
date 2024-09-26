/*
Solution:

Time: O(n) | Space: O(n)

Runtime: 57 ms | 98.63%
Memory: 47.79 MB | 10.75%

- n: # of operations called on 'MyCalendar'
*/

struct Node {
    Node* left;
    Node* right;
    int start;
    int end;

    Node(): start(-1), end(-1), left(NULL), right(NULL) {}
    Node(int s, int e): start(s), end(e), left(NULL), right(NULL) {}
};

class MyCalendar {
public:
    Node* root;

    MyCalendar() {
        root = NULL;
    }
    
    bool book(int start, int end) {
        bool flag = true;
        root = insert(root, start, end, &flag);
        return flag;
    }

    Node* insert(Node* root, int start, int end, bool* flag) {
        if (root == NULL) {
            root = new Node(start, end);
        } else {
            if (start < root->start && end <= root->start) {
                root->left = insert(root->left, start, end, flag);
            } else if (start >= root->end) {
                root->right = insert(root->right, start, end, flag);
            } else {
                *flag = false;
            }
        }
        return root;
    }
};

/**
 * Your MyCalendar object will be instantiated and called as such:
 * MyCalendar* obj = new MyCalendar();
 * bool param_1 = obj->book(start,end);
 */