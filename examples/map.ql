let map = fn(arr, f) {
  let iter = fn(arr, accumulated) {
    if len(arr) == 0 {
      accumulated
    } else {
        iter(rest(arr), append(accumulated, f(arr[0])));
    }
  };

  iter(arr, []);
};

let arr = [1, 2, 3, 4];
let double = fn(x) { x * 2 };
println(map(arr, double));
