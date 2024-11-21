use std::{thread, time};
use array2d::{Array2D};
use fastrand::bool;

fn main() {
    // taille de la grille
    let rows: usize = 25;
    let columns: usize = 40;

    // génère une grille de 50x50 en la remplissant avec le retour de la fonction bool()
    let mut grid = Array2D::filled_by_row_major(bool, rows, columns);

    let mut turn = 0;
    let max_turns = 1000;

    let mut alive_cells: u32;

    while turn < max_turns {
        let now = time::Instant::now();

        alive_cells = 0;

        // on crée une copie de la grille
        let mut new_grid = grid.clone();

        // on met à jour les cellules
        let mut row_index: usize = 0;
        for row_iter in grid.rows_iter() {
            let mut column_index: usize = 0;
            for cell in row_iter {
                // on compte le nombre de cellules voisines vivantes
                let mut live_neighbors: u8 = 0;

                // row - 1
                if
                    row_index > 0 &&
                    column_index > 0 &&
                    grid[(row_index - 1, column_index - 1)]
                {
                    live_neighbors += 1;
                }

                if
                    row_index > 0 &&
                    grid[(row_index - 1, column_index)]
                {
                    live_neighbors += 1;
                }

                if
                    row_index > 0 &&
                    column_index + 1 < columns &&
                    grid[(row_index - 1, column_index + 1)]
                {
                    live_neighbors += 1;
                }

                // row
                if
                    column_index > 0 &&
                    grid[(row_index, column_index - 1)]
                {
                    live_neighbors += 1;
                }

                if
                    column_index + 1 < columns &&
                    grid[(row_index, column_index + 1)]
                {
                    live_neighbors += 1;
                }

                // row + 1
                if
                    row_index + 1 < rows &&
                    column_index > 0 &&
                    grid[(row_index + 1, column_index - 1)]
                {
                    live_neighbors += 1;
                }

                if
                    row_index + 1 < rows &&
                    grid[(row_index + 1, column_index)]
                {
                    live_neighbors += 1;
                }

                if
                    row_index + 1 < rows &&
                    column_index + 1 < columns &&
                    grid[(row_index + 1, column_index + 1)]
                {
                    live_neighbors += 1;
                }

                // la cellule est vivante si elle a 3 voisins vivants,
                // ou si elle est vivante et a 2 voisins vivants.
                let is_cell_alive =
                    live_neighbors == 3 ||
                    (*cell && live_neighbors == 2);

                if is_cell_alive {
                    alive_cells += 1;
                }

                new_grid[(row_index, column_index)] = is_cell_alive;

                    column_index += 1;
            }
            row_index += 1;
        }

        // on affecte la nouvelle grille
        grid = new_grid;

        turn += 1;

        let elapsed = now.elapsed();

        // on clean le terminal
        print!("{esc}c", esc = 27 as char);

        // on imprime des stats
        println!(
            "Grille : {}x{} - Tour : {} / {} - Cellules : {} - Temps : {:.2?}",
            rows, columns, turn, max_turns, alive_cells, elapsed
        );

        // on imprime la grille
        for row_iter in grid.rows_iter() {
            let mut row: String = "".to_owned();
            for element in row_iter {
                row.push_str(if *element {"▓▓"} else {"░░"});
            }
            println!("{}", row);
        }

        // on attend
        thread::sleep(time::Duration::from_millis(100));
    }
}
