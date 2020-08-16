/*
pika! pika!     >   increment the data pointer
pika. pika.     <   decrement the data pointer
pika. pika!     +   increment the byte at pointer
pika! pika.     -   decrement the byte at pointer
pikachu!        .   output the byte at pointer
pikachu?        ,   input of one byte into pointer
pika pi?        [   Jump forward past the matching ] if the byte at the pointer is zero.
pika pi!        ]   if pointer is nonzero, jump to matching pika
*/
#[macro_export]
macro_rules! pikachu {
    (@start $($pika:tt)*) => {{
        type OneByte = u8;
        const MEM_SIZE: usize = 300;
        use std::io::{self, prelude::*, Result, Error, ErrorKind};
        fn pikapika() -> Result<Vec<OneByte>> {
            fn _err() -> Error {
                Error::new(ErrorKind::Other, String::from("ran out of input"))
            }
            fn _inc(a: &mut [u8], i: usize) {
                let c = &mut a[i];
                *c = c.wrapping_add(1);
            }
            fn _dec(a: &mut [u8], i: usize) {
                let c = &mut a[i];
                *c = c.wrapping_sub(1);
            }
            fn _fwd(i:usize) -> usize {
                (i + 1) % MEM_SIZE
            }
            fn _bwd(i:usize) -> usize {
                if i == 0 { MEM_SIZE } else { i - 1 }
            }
            let _sin = &mut io::stdin();
            let _sout = &mut io::stdout();

            // let mut _mem:Vec<OneByte> = Vec::with_capacity(MEM_SIZE);
            // _mem.extend(std::iter::repeat(0).take(MEM_SIZE));
            let mut _mem:Vec<OneByte> = vec![0;MEM_SIZE];
            let mut _i = 0;
            {
                let _mem = &mut *_mem;
                pikachu!(@step (_mem, _i, _inc, _dec, _fwd, _bwd, _sin, _sout, _err); ($($pika)*));
            }
            Ok(_mem)
        }
        pikapika()
    }};

    // extraction point
    (@step $symbols:tt; ()) => {};

    //pika! pika!     >   increment the data pointer
    (@step ($mem:expr, $i:expr, $inc:expr, $dec:expr, $fwd:expr, $bwd:expr, $sin:expr, $sout:expr, $err:expr);
        (pika! pika! $($tails:tt)*))
    => {
        $i = $fwd($i);
        pikachu!(@step ($mem, $i, $inc, $dec, $fwd, $bwd, $sin, $sout, $err); ($($tails)*));
    };
    //pika. pika.     <   decrement the data pointer
    (@step ($mem:expr, $i:expr, $inc:expr, $dec:expr, $fwd:expr, $bwd:expr, $sin:expr, $sout:expr, $err:expr);
        (pika. pika. $($tails:tt)*))
    => {
        $i = $bwd($i);
        pikachu!(@step ($mem, $i, $inc, $dec, $fwd, $bwd, $sin, $sout, $err); ($($tails)*));
    };
    //pika. pika!     +   increment the byte at pointer
    (@step ($mem:expr, $i:expr, $inc:expr, $dec:expr, $fwd:expr, $bwd:expr, $sin:expr, $sout:expr, $err:expr);
        (pika. pika! $($tails:tt)*))
    => {
        $inc($mem, $i);
        pikachu!(@step ($mem, $i, $inc, $dec, $fwd, $bwd, $sin, $sout, $err); ($($tails)*));
    };
    //pika! pika.     -   decrement the byte at pointer
    (@step ($mem:expr, $i:expr, $inc:expr, $dec:expr, $fwd:expr, $bwd:expr, $sin:expr, $sout:expr, $err:expr);
        (pika! pika. $($tails:tt)*))
    => {
        $dec($mem, $i);
        pikachu!(@step ($mem, $i, $inc, $dec, $fwd, $bwd, $sin, $sout, $err); ($($tails)*));
    };
    //pikachu!        .   output the byte at pointer
    (@step ($mem:expr, $i:expr, $inc:expr, $dec:expr, $fwd:expr, $bwd:expr, $sin:expr, $sout:expr, $err:expr);
        (pikachu! $($tails:tt)*))
    => {
        $sout.write_all(&$mem[$i .. $i+1])?;
        pikachu!(@step ($mem, $i, $inc, $dec, $fwd, $bwd, $sin, $sout, $err); ($($tails)*));
    };
    //pikachu?        ,   input of one byte into pointer
    (@step ($mem:expr, $i:expr, $inc:expr, $dec:expr, $fwd:expr, $bwd:expr, $sin:expr, $sout:expr, $err:expr);
        (pikachu? $($tails:tt)*))
    => {
        match $sin.read(&mut $a[$i .. $i+1]) {
            Ok(0) => Err($err()),
            ok @ Ok(..) => ok,
            err @ Err(..) => err
        }?;
        pikachu!(@step ($mem, $i, $inc, $dec, $fwd, $bwd, $sin, $sout, $err); ($($tails)*));
    };
    //pika pi?        [   Jump forward past the matching ] if the byte at the pointer is zero.
    //pika pi!        ]   if pointer is nonzero, jump to matching pika
    (@step ($mem:expr, $i:expr, $inc:expr, $dec:expr, $fwd:expr, $bwd:expr, $sin:expr, $sout:expr, $err:expr);
        (pika pi? $($tails:tt)*))
    => {
        while $mem[$i] != 0 {
            pikachu!(@inner_loop ($mem, $i, $inc, $dec, $fwd, $bwd, $sin, $sout, $err);
                (/*nesting loop count*/);
                (/*cached code*/);
                ($($tails)*));
        }
        pikachu!(@end_loop ($mem, $i, $inc, $dec, $fwd, $bwd, $sin, $sout, $err); (); ($($tails)*));
    };
    // loop goes deeper, loop code cached
    (@inner_loop $symbols:tt;
        ($($depth:tt)*);
        ($($buf:tt)*);
        (pika pi? $($tails:tt)*))
    => {
        pikachu!(@inner_loop $symbols; (@ $($depth)*); ($($buf)* pika pi?); ($($tails)*));
    };
    // loop goes shallower, loop code cached
    (@inner_loop $symbols:tt;
        (@ $($depth:tt)*);
        ($($buf:tt)*);
        (pika pi! $($tails:tt)*))
    => {
        pikachu!(@inner_loop $symbols; ($($depth)*); ($($buf)* pika pi!); ($($tails)*));
    };
    // handle cached code
    (@inner_loop $symbols:tt;
        ();
        ($($buf:tt)*);
        (pika pi! $($tails:tt)*))
    => {
        // the most outside loop is done, handle the cached code
        pikachu!(@step $symbols; ($($buf)*));
    };
    // any code except loop will be cached here
    // pika(?) pika(?)
    (@inner_loop $symbols:tt;
        $depth:tt;
        ($($buf:tt)*);
        (pika $op0:tt pika $op1:tt $($tail:tt)*))
    => {
        pikachu!(@inner_loop $symbols; $depth; ($($buf)* pika $op0 pika $op1); ($($tail)*));
    };
    // pikachu(?)
    (@inner_loop $symbols:tt;
        $depth:tt;
        ($($buf:tt)*);
        (pikachu $op:tt $($tail:tt)*))
    => {
        pikachu!(@inner_loop $symbols; $depth; ($($buf)* pikachu $op); ($($tail)*));
    };
    // end the loop
    (@end_loop $symbols:tt;
        ();
        (pika pi! $($tail:tt)*))
    => {
        pikachu!(@step $symbols; ($($tail)*));
    };
    // Enter nested loop.
    (@end_loop $symbols:tt;
        ($($depth:tt)*);
        (pika pi? $($tail:tt)*))
    => {
        pikachu!(@end_loop $symbols; (@ $($depth)*); ($($tail)*));
    };
    // Exit nested loop.
    (@end_loop $symbols:tt;
        (@ $($depth:tt)*);
        (pika pi! $($tail:tt)*))
    => {
        pikachu!(@end_loop $symbols; ($($depth)*); ($($tail)*));
    };
    // Not a loop code.
    (@end_loop $symbols:tt;
        ($($depth:tt)*);
        (pika $op0:tt pika $op1:tt $($tail:tt)*))
    => {
        pikachu!(@end_loop $symbols; ($($depth)*); ($($tail)*));
    };
    (@end_loop $symbols:tt;
        ($($depth:tt)*);
        (pikachu $op:tt $($tail:tt)*))
    => {
        pikachu!(@end_loop $symbols; ($($depth)*); ($($tail)*));
    };

    //entry
    ($($pikapikapikachu:tt)*) => {
        pikachu!(@start $($pikapikapikachu)*)
    };
}
