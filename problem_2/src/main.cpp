#include <time.h>
#include <iostream>
#include <cassert>

int sum_evenfib(int limit) {
    int sum=0, a=1 ,b=2,_c=0,d=2;
    while(d < limit) {
        sum += d;
        _c = a + (2 * b);
        d = (2 * _c) - b;
        b = d;
        a = _c;
    }
    return sum;
}


int main(void) {
    clock_t tStart = clock();
    /* Do your stuff here */
    const auto answer = sum_evenfib(4000001);
    // const auto answer = sum_multiples(3,999) + sum_multiples(5,999) - sum_multiples(15,999);
    std::cout <<  "Time taken: " << ((double)(clock() - tStart)/CLOCKS_PER_SEC) << std::endl;
    std::cout << "answer is: " << answer << std::endl;
    assert(answer == 4613732);
    return 0;
}

