/*
// Definition for a Node.
class Node {
public:
    int val;
    vector<Node*> children;

    Node() {}

    Node(int _val) {
        val = _val;
    }

    Node(int _val, vector<Node*> _children) {
        val = _val;
        children = _children;
    }
};
*/

/*
Solution: DFS

Time: O(n) | Space: O(n)

Runtime: 4 ms | 98.78%
Memory: 15.24 MB | 49.47%

- n: the number of nodes in the tree
*/

class Solution {
public:
    vector<int> postorder(Node* root) {
        vector<int> res;

        traverse(&res, root);

        return res;
    }

    void traverse(vector<int>* res, Node* root) {
        if (root == NULL) {
            return;
        } else {
            for (auto &node: root->children) {
                traverse(res, node);
            }
            res->push_back(root->val);
        }
    }
};