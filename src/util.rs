fn border_array(text: String) -> Vec<usize> {
    let x = text.into_bytes();
    let mut ba = vec![0; x.len()];

    ba[0] = 0;
    for i in 1..x.len() {
        let mut b = ba[i - 1];

        while b > 0 && x[i] != x[b] {
            b = ba[b - 1];
        }

        ba[i] = if x[i] == x[b] { b + 1 } else { 0 };
    }

    return ba;
}

#[cfg(test)]
mod tests {
    use crate::util::border_array;

    #[test]
    fn correctly_computes_example() {
        let ba = border_array("abcabac".to_string());
        assert_eq!(ba, vec![0, 0, 0, 1, 2, 1, 0])
    }
}
