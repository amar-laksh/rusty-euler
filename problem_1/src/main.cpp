#include <time.h>
#include <iostream>
#include  <cassert>

int sum_multiples(int multiple, int limit=999){ 
    return multiple * (limit/multiple) * (limit/multiple + 1) / 2;
}


int main(void) {
    clock_t tStart = clock();
    /* Do your stuff here */
    const auto answer = sum_multiples(3) + sum_multiples(5) - sum_multiples(15);
    std::cout <<  "Time taken: " << ((double)(clock() - tStart)/CLOCKS_PER_SEC) << std::endl;
    std::cout << "answer is: " << answer << std::endl;
    assert(answer == 233168);
    return 0;
}

