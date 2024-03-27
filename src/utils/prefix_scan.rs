use crate::utils::cwslice::UnsafeSlice;

fn par_scan_up<'a>(xs: &'a [usize], aux: &UnsafeSlice<'a, usize>, aux_offset: usize) -> usize {
    if xs.len() == 1 {
        xs[0]
    } else {
        let m = xs.len() / 2;
        let (xs_left, xs_right) = xs.split_at(m);
        let (left, right) = rayon::join(
            || par_scan_up(xs_left, aux, aux_offset),
            || par_scan_up(xs_right, aux, aux_offset + m),
        );
        unsafe {
            aux.write(aux_offset + m - 1, left);
        }
        left + right
    }
}

fn par_scan_down(
    aux: &[usize],
    res: &UnsafeSlice<'_, usize>,
    ps: usize,
    res_offset: usize,
    res_size: usize,
) {
    if res_size == 1 {
        unsafe {
            res.write(res_offset, ps);
        }
    } else {
        let m = res_size / 2;
        let (aux_left, aux_right) = aux.split_at(m);
        rayon::join(
            || par_scan_down(aux_left, res, ps, res_offset, m),
            || {
                par_scan_down(
                    aux_right,
                    res,
                    ps + aux[m - 1],
                    res_offset + m,
                    res_size - m,
                )
            },
        );
    }
}

pub fn par_scan(xs: &[usize]) -> (usize, Vec<usize>) {
    let mut ell = vec![0; xs.len() - 1];
    let ell_slice = UnsafeSlice::new(&mut ell);
    let mut res = vec![0; xs.len()];
    let res_slice = UnsafeSlice::new(&mut res);

    let total = par_scan_up(xs, &ell_slice, 0);
    par_scan_down(&ell, &res_slice, 0, 0, xs.len());
    (total, res)
}

mod test {
    #[test]
    fn prefix_sum() {
        use super::par_scan;
        use rand::seq::SliceRandom;

        let sample_size = 1_000;
        let mut population = (0..sample_size).collect::<Vec<usize>>();
        let mut rng = rand::thread_rng();
        population.shuffle(&mut rng);

        let mut acc = 0;
        let mut seq_ps: Vec<usize> = Vec::with_capacity(sample_size);
        population.iter().for_each(|elm| {
            seq_ps.push(acc);
            acc += elm;
        });

        let (ps, partials) = par_scan(&population);

        assert_eq!(acc, ps);
        assert_eq!(&seq_ps, &partials);
    }
}
