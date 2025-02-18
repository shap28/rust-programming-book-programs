#include <iostream>
using namespace std;

int main(){
    int i =0;
    int j =0;
    for (i=10;i>=0;i--){
        for (j=i; j<=10;j++){
            cout<<j;
        }
        cout<<endl;
    }
}