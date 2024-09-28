/*
Solution:

Time: O(1) | Space: O(1)

Runtime: 14 ms | 95.58%
Memory: 22.96 MB | 11.65%


*/

struct node {
    int val;
    node* prev;
    node* next;

    node(int val): val(val) {}
};

class MyCircularDeque {
public:
    int max;
    int current;

    node* head;
    node* tail;

    MyCircularDeque(int k) {
        max = k;
        current = 0;    
        head = NULL;
        tail = NULL;
    }
    
    bool insertFront(int value) {
        if (head == NULL) {
            node* tmp = new node(value);

            tmp->next = tmp;
            tmp->prev = tmp;

            head = tmp;
            tail = tmp;

            current += 1;

            return true;
        } else if (current < max) {
            node* tmp = new node(value);

            tmp->next = head;
            tmp->prev = tail;
            tail->next = tmp;
            head->prev = tmp;

            head = tmp;

            current += 1;
            
            return true;
        } else {
            return false;
        }
    }
    
    bool insertLast(int value) {
        if (head == NULL) {
            node* tmp = new node(value);

            tmp->next = tmp;
            tmp->prev = tmp;

            head = tmp;
            tail = tmp;

            current += 1;

            return true;
        } else if (current < max) {
            node* tmp = new node(value);

            tmp->next = head;;
            tmp->prev = tail;
            tail->next = tmp;
            head->prev = tmp;

            tail = tmp;

            current += 1;
            
            return true;
        } else {
            return false;
        }
    }
    
    bool deleteFront() {
        if (head == NULL) {
            return false;
        } else if (current == 1) {
            node* tmp = head;

            head = NULL;
            tail = NULL;

            current -= 1;

            delete tmp;

            return true;
        }  else {
            node* tmp = head;

            head = head->next;
            head->prev = tail;
            tail->next = head;

            current -= 1;

            delete tmp;

            return true;
        }
    }
    
    bool deleteLast() {
        if (head == NULL) {
            return false;
        } else if (current == 1) {
            node* tmp = tail;

            head = NULL;
            tail = NULL;

            current -= 1;

            delete tmp;

            return true;
        }   else {
            node* tmp = tail;

            tail = tail->prev;
            head->prev = tail;
            tail->next = head;

            current -= 1;

            delete tmp;

            return true;
        }
    }
    
    int getFront() {
        if (isEmpty()) {
            return -1;
        } else {
            return head->val;
        }
    }
    
    int getRear() {
        if (isEmpty()) {
            return -1;
        } else {
            return tail->val;
        }
    }
    
    bool isEmpty() {
        return current == 0;
    }
    
    bool isFull() {
        return current == max;
    }
};

/**
 * Your MyCircularDeque object will be instantiated and called as such:
 * MyCircularDeque* obj = new MyCircularDeque(k);
 * bool param_1 = obj->insertFront(value);
 * bool param_2 = obj->insertLast(value);
 * bool param_3 = obj->deleteFront();
 * bool param_4 = obj->deleteLast();
 * int param_5 = obj->getFront();
 * int param_6 = obj->getRear();
 * bool param_7 = obj->isEmpty();
 * bool param_8 = obj->isFull();
 */