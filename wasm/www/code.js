const fibonacci = `int fibonacci(int a) {
  if (a < 2) {
    return a;
  }
  return fibonacci(a - 1) + fibonacci(a - 2);
}
void main() {
  print(fibonacci(10));
}
`;

const bubbleSort = `void bubbleSort(int a[], int len) {
   int i = len - 1;
   while (i > 0) {
      int j = 0;
      while (j < i) {
         if (a[j] > a[j + 1]) {
            int tem = a[j];
            a[j] = a[j + 1];
            a[j + 1] = tem;
         }
         j = j + 1;
      }
      i = i- 1;
   }
}
void main() {
   int a[5] = {4, 10, 1, 7, 2};
   bubbleSort(a, 5);
   print(a);
}

`;

const insertSort = `void insertSort(int a[], int len) {
  int i = 1;
  while (i < len) {
    int target = a[i];
    int j = i - 1;
    while (j >= 0 && a[j] > target) {
      a[j + 1] = a[j];
      j = j - 1;
    }
   a[j + 1] = target;
   i = i + 1;
  }
}
void main() {
  int a[8] = { 4, 10, 1, 7, 2, 8, 9, 2 };
  insertSort(a, 8);
  print(a);
}
`

export default {
    fibonacci,
    bubbleSort,
    insertSort
}
