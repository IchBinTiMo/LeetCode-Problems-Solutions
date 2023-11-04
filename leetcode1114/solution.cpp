class Foo {
    std::condition_variable cv;
    std::mutex mtx;
    int current;
public:
    Foo() {
        current = 1;
    }

    void first(function<void()> printFirst) {
        // printFirst() outputs "first". Do not change or remove this line.
        printFirst();
        current = 2;
        cv.notify_all();
    }

    void second(function<void()> printSecond) {
        
        // printSecond() outputs "second". Do not change or remove this line.
        std::unique_lock<std::mutex> lock(mtx);
        cv.wait(lock, [&](){return current == 2;});
        printSecond();
        current = 3;
        cv.notify_all();
    }

    void third(function<void()> printThird) {
        
        // printThird() outputs "third". Do not change or remove this line.
        std::unique_lock<std::mutex> lock(mtx);
        cv.wait(lock, [&](){return current == 3;});
        printThird();
    }
};