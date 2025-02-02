#include <iostream>

using namespace std;

// A simple function to fill an array with the values 0..(size-1)
void fillArray(int* arr, int size) {
    for (int i = 0; i < size; ++i) {
        arr[i] = i;
    }
}

int main() {
    // Stack allocation (automatically managed)
    int x = 10;
    cout << "Stack-allocated variable x = " << x << endl;

    // Manual heap allocation
    int size = 1000000;
    int* arr = new int[size];  // Allocate an array of 1,000,000 ints

    // Initialize the newly allocated memory
    fillArray(arr, size);

    // Print some values to confirm the array is filled
    cout << "First element of arr: " << arr[0]
         << ", last element of arr: " << arr[size - 1] << endl;

    // Manually deallocate the array to avoid a memory leak
    delete[] arr;

    // Example of a potential memory leak if uncommented:
    // int* leak = new int[500000];
    // If never freed with delete[] leak;, it remains a leak.

    return 0;
}
