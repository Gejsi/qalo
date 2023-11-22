let reduce = fn(arr, initial, f) {
  let iter = fn(arr, result) {
    if len(arr) == 0 {
      result
    } else {
      iter(rest(arr), f(result, arr[0]));
    }
  };

  iter(arr, initial);
};

let sum = fn(arr) {
  return reduce(arr, 0, fn(initial, el) { initial + el });
};

println(sum([1, 2, 3, 4, 5]));
