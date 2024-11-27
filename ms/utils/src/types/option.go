package types

type Option[T any] struct {
	value  T
	isSome bool
}

func Some[T any](value T) Option[T] {
	return Option[T]{value: value, isSome: true}
}

func None[T any]() Option[T] {
	return Option[T]{isSome: false}
}

func (o Option[T]) IsSome() bool { return o.isSome }
func (o Option[T]) IsNone() bool { return !o.isSome }
func (o Option[T]) Unwrap() T {
	if o.isSome {
		return o.value
	}
	panic("called `Option.Unwrap()` on a `None` value")
}
