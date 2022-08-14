#![allow(unused_parens)]

macro_rules! lisp_let {
	(($n:ident $v:tt)) => {
		let $n = lisp!{$v};
	};
	(($n:ident)) => {
		let $n;
	};
}

macro_rules! lisp_expr {
	((let ($($lets:tt)*) $($block:tt)*)) => {
		{
			$(
				lisp_let!{$lets};
			)*
			lisp!{$($block)*}
		}
	};
	(($fn:ident $($x:tt)+)) => {
		$fn($(lisp!($x),)+)
	};
	(($v:expr)) => {
		$v
	};
}

macro_rules! lisp {
	((defun $n:ident ($($pn:ident: $pt:ty),*) -> $t:ty => $($block:tt)*)) => {
		fn $n($($pn: $pt),*) -> $t {
			lisp!{$($block)*}
		}
	};
	($($item:tt)+) => {
		{
			$(
				#[allow(unused_variables)]
				let v = lisp_expr!{$item};
			)+
			v
		}
	};
	() => {
		()
	}
}

lisp! {
	(defun add (a: u32, b: u32) -> u32 => (
		(a + b)
		))
}

lisp! {
	(defun main () -> () =>
		(let ((foo (add (1) (200))))
			((println!("{}", foo))))
	)
}
