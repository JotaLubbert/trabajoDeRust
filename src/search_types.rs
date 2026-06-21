pub fn brillo_cercano(lista_brillos: &[f32], objetivo: f32) -> usize {
    let idx: usize = lista_brillos.partition_point(|&b| b < objetivo);
    if idx == 0 {
        return 0;
    }
    if idx == lista_brillos.len() {
        return lista_brillos.len() - 1;
    }
    let anterior = lista_brillos[idx - 1];
    let siguiente = lista_brillos[idx];
    if (objetivo - anterior).abs() <= (siguiente - objetivo).abs() {
        idx - 1
    } else {
        idx
    }
}
