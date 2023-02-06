use super::*;

impl From<RankAux<layout::Uninit>> for RankAux<layout::Accumulated> {
    fn from(mut flat: RankAux<layout::Uninit>) -> RankAux<layout::Accumulated> {
        use fenwicktree::Nodes;

        let mut sum = 0;
        for acc in flat.ub[1..].iter_mut() {
            sum += *acc;
            *acc = sum;
        }

        for q in 0..flat.ub.nodes() {
            let lo = flat.lo_mut(q);

            let mut sum = 0;
            for L1L2(acc) in lo[1..].iter_mut() {
                let cur = *acc & L1L2::L1;
                *acc = (*acc & !L1L2::L1) | sum;
                sum += cur;
            }
        }

        RankAux { ub: flat.ub, lb: flat.lb, _lb_layout: PhantomData }
    }
}