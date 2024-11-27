package types

type Result[T any, E any] struct {
	value T
	err   E
	isOk  bool
}

func Ok[T any, E any](value T) Result[T, E] {
	return Result[T, E]{value: value, isOk: true}
}

func Err[T any, E any](err E) Result[T, E] {
	return Result[T, E]{err: err, isOk: false}
}

func (r Result[T, E]) IsOk() bool  { return r.isOk }
func (r Result[T, E]) IsErr() bool { return !r.isOk }
func (r Result[T, E]) Unwrap() T {
	if r.isOk {
		return r.value
	}
	panic("called `Result.Unwrap()` on an `Err` value")
}
