#include <bits/stdc++.h>
using namespace std;

int main()
{
    cout << "Hello, World!\n";

    cout << "for loop\n";
    for (int i = 0; i < 5; i++)
    {
        cout << i << "\n";
    }

    cout << "while loop\n";
    int j = 5;
    while (j > 0)
    {
        cout << j << endl;
        j--;
    }

    cout << "do while loop\n";
    int num;
    do
    {
        cout << "Positive Num: ";
        cin >> num;
    } while (num < 1);
    
}