class FooBar {
private:
    std::condition_variable cv;
    std::mutex mtx;
    int n;
    int current;

public:
    FooBar(int n) {
        this->n = n;
        this->current = 0;
    }

    void foo(function<void()> printFoo) {
        
        for (int i = 0; i < n; i++) {
            std::unique_lock<std::mutex> lock(mtx);
            cv.wait(lock, [&](){return this->current == 0;});
        	// printFoo() outputs "foo". Do not change or remove this line.
        	printFoo();
            this->current = 1;
            lock.unlock();
            cv.notify_all();
        }
    }

    void bar(function<void()> printBar) {
        
        for (int i = 0; i < n; i++) {
            std::unique_lock<std::mutex> lock(mtx);
            cv.wait(lock, [&](){return this->current == 1;});
        	// printBar() outputs "bar". Do not change or remove this line.
        	printBar();
            this->current = 0;
            lock.unlock();
            cv.notify_all();
        }
    }
};