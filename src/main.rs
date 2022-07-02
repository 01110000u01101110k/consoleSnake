use rand::{thread_rng, Rng};
use std::io::{self, Write};
use std::time::Duration;
use windows::Win32::Foundation::BOOL;
use windows::Win32::System::Console::GetConsoleScreenBufferInfo;
use windows::Win32::System::Console::GetStdHandle;
use windows::Win32::System::Console::ScrollConsoleScreenBufferW;
use windows::Win32::System::Console::SetConsoleCursorInfo;
use windows::Win32::System::Console::SetConsoleCursorPosition;
use windows::Win32::System::Console::SetConsoleTextAttribute;
use windows::Win32::System::Console::CHAR_INFO;
use windows::Win32::System::Console::CONSOLE_CURSOR_INFO;
use windows::Win32::System::Console::CONSOLE_SCREEN_BUFFER_INFO;
use windows::Win32::System::Console::COORD;
use windows::Win32::System::Console::SMALL_RECT;
use windows::Win32::System::Console::STD_OUTPUT_HANDLE;

// список цветов консоли
const BLACK: u16 = 0; // Черный

const DARK_BLUE: u16 = 1; // Синий

const GREEN: u16 = 2; // Зеленый

const BLUE: u16 = 3; // Голубой

const RED: u16 = 4; // Красный

const VIOLET: u16 = 5; // Лиловый

const YELLOW: u16 = 6; // Желтый

const WHITE: u16 = 7; // Белый

const GREY: u16 = 8; // Серый

const LIGHT_BLUE: u16 = 9; // Светло-синий

const LIGHT_GREEN: char = 'A'; // Светло-зеленый

const LIGHT_DARK_BLUE: char = 'B'; // Светло-голубой

const LIGHT_RED: char = 'C'; // Светло-красный

const LIGHT_PURPLE: char = 'D'; // Светло-лиловый

const LIGHT_YELLOW: char = 'E'; // Светло-желтый

const BRIGHT_WHITE: char = 'F'; // Ярко-белый

const CONSOLE_WIDTH: i32 = 118; // начальная ширина консоли
const CONSOLE_HEIGHT: i32 = 28; // начальная высота консоли

enum Directions {
    Up,
    Down,
    Left,
    Right,
}

struct Move {
    direction: Directions,
}

impl Move {
    fn new() -> Self {
        Self {
            direction: Directions::Left,
        }
    }

    fn change_direction(&mut self, direction: Directions) {
        self.direction = direction;
    }
}

#[derive(Copy, Clone, Debug)]
struct SnakePiece {
    position_x: i32,
    position_y: i32,
}

impl SnakePiece {
    fn new() -> Self {
        Self {
            position_x: 0,
            position_y: 0,
        }
    }
    fn set_position(x: i32, y: i32) -> Self {
        Self {
            position_x: x,
            position_y: y,
        }
    }
}

fn text_center(text: String) {
    let indent = " ";
    let mut indent_row = "".to_string();

    let mut index_row_element = 0;

    while index_row_element <= CONSOLE_WIDTH {
        indent_row += &indent;
        index_row_element += 1;
    }

    let mut render_step = 0;

    while render_step < CONSOLE_HEIGHT / 2 {
        if render_step != CONSOLE_HEIGHT / 2 - 1 {
            let stdout = io::stdout(); // get the global stdout entity
            let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
            writeln!(handle, "{}", indent_row); // add `?` if you care about errors here
        } else {
            let slice = &indent_row[0..indent_row.len() / 2 - text.len() / 2 as usize];

            let stdout = io::stdout(); // get the global stdout entity
            let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
            writeln!(handle, "{}{}", slice, text); // add `?` if you care about errors here
        }

        render_step += 1;
    }
}

fn spawn_snake(snake: &mut Vec<SnakePiece>) {
    let mut spawn_snake_piece = 1;

    while spawn_snake_piece < 5 {
        let piece =
            SnakePiece::set_position(CONSOLE_WIDTH / 2 - spawn_snake_piece, CONSOLE_HEIGHT / 2);
        snake.push(piece);

        spawn_snake_piece += 1;
    }
}

fn hide_cursor() {
    unsafe {
        // скрываю курсор
        let cursor = CONSOLE_CURSOR_INFO {
            dwSize: 1,
            bVisible: BOOL(0),
        };
        SetConsoleCursorInfo(GetStdHandle(STD_OUTPUT_HANDLE), &cursor);
    }
}

fn set_text_color(color: u16) {
    unsafe {
        SetConsoleTextAttribute(GetStdHandle(STD_OUTPUT_HANDLE), color);
    }
}

fn clear_console() {
    unsafe {
        let console_handle = GetStdHandle(STD_OUTPUT_HANDLE);
        let mut csbi = CONSOLE_SCREEN_BUFFER_INFO::default();
        let mut scroll_rect = SMALL_RECT::default();
        let mut scroll_target = COORD::default();
        let fill = CHAR_INFO::default();

        // Get the number of character cells in the current buffer.
        if GetConsoleScreenBufferInfo(console_handle, &mut csbi) == false {
            return;
        }

        // Scroll the rectangle of the entire buffer.
        scroll_rect.Left = 0;
        scroll_rect.Top = 0;
        scroll_rect.Right = csbi.dwSize.X;
        scroll_rect.Bottom = csbi.dwSize.Y;

        // Scroll it upwards off the top of the buffer with a magnitude of the entire height.
        scroll_target.X = 0;
        scroll_target.Y = 0;

        // Do the scroll
        ScrollConsoleScreenBufferW(
            console_handle,
            &scroll_rect,
            &SMALL_RECT::default(),
            scroll_target,
            &fill,
        );

        // Move the cursor to the top left corner too.
        csbi.dwCursorPosition.X = 0;
        csbi.dwCursorPosition.Y = 0;

        SetConsoleCursorPosition(console_handle, csbi.dwCursorPosition);
    }
}

fn spawn_food(food_position_x: &mut i32, food_position_y: &mut i32) {
    let mut rng = thread_rng();
    *food_position_x = rng.gen_range(2..CONSOLE_WIDTH - 2);
    *food_position_y = rng.gen_range(2..CONSOLE_HEIGHT - 2);
}

fn main() {
    hide_cursor();
    let mut graphics = "".to_string();

    let mut snake: Vec<SnakePiece> = vec![];

    spawn_snake(&mut snake);

    let mut snake_len = &snake.len() - 1;

    let mut direction = Move::new();

    let mut food_position_x = 0;
    let mut food_position_y = 0;

    let mut is_game_over = true;

    let mut update_render = false;

    let start_game_text = "Press 'Enter' to start new game.".to_string();

    spawn_food(&mut food_position_x, &mut food_position_y);

    set_text_color(WHITE);
    text_center(start_game_text);

    'main: loop {
        snake_len = &snake.len() - 1;
        //let now = std::time::Instant::now(); // начало замера времени. Для замера производительности
        unsafe {
            if windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(87) | windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(38)
                == (1 | -32767 | -32768)
            {
                match direction.direction {
                    Directions::Right | Directions::Left => {
                        direction.change_direction(Directions::Up)
                    }
                    _ => (),
                }
            } else if windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(83) | windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(40)
                == (1 | -32767 | -32768)
            {
                match direction.direction {
                    Directions::Right | Directions::Left => {
                        direction.change_direction(Directions::Down)
                    }
                    _ => (),
                }
            } else if windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(65) | windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(37)
                == (1 | -32767 | -32768)
            {
                match direction.direction {
                    Directions::Up | Directions::Down => {
                        direction.change_direction(Directions::Left)
                    }
                    _ => (),
                }
            } else if windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(68) | windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(39)
                == (1 | -32767 | -32768)
            {
                match direction.direction {
                    Directions::Up | Directions::Down => {
                        direction.change_direction(Directions::Right)
                    }
                    _ => (),
                }
            } else if windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(13)
                == (1 | -32767 | -32768)
                && is_game_over
            {
                is_game_over = false;
                update_render = true;
                snake = vec![];
                spawn_snake(&mut snake);
            } else if windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(32) | windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(27)
                == (1 | -32767 | -32768)
                && !is_game_over
            {
                update_render = !update_render;

                //std::io::stdin().read_line(&mut String::new()).unwrap();
            }
        }

        // рендер
        if update_render {
            if !is_game_over {
                if graphics.len() != 0 {
                    graphics = "".to_string();
                }

                match direction.direction {
                    /*
                        snake.push(snake[0]); // push добавляет элемент в конец вектора
                        snake.remove(0); // remove удаляет элемент из вектора
                        snake.insert(0, 0); // позволяет добавить элемент в любую часть вектора
                    */
                    Directions::Up => {
                        snake[snake_len].position_x = snake[0].position_x;
                        snake[snake_len].position_y = snake[0].position_y - 1;
                    }
                    Directions::Down => {
                        snake[snake_len].position_x = snake[0].position_x;
                        snake[snake_len].position_y = snake[0].position_y + 1;
                    }
                    Directions::Left => {
                        snake[snake_len].position_y = snake[0].position_y;
                        snake[snake_len].position_x = snake[0].position_x - 1;
                    }
                    Directions::Right => {
                        snake[snake_len].position_y = snake[0].position_y;
                        snake[snake_len].position_x = snake[0].position_x + 1;
                    }
                }

                snake.rotate_right(1);

                let mut i = 0;

                let mut x = 0;
                let mut y = 0;

                while i < CONSOLE_WIDTH * CONSOLE_HEIGHT {
                    let mut is_empty = true;

                    if food_position_x == x && food_position_y == y {
                        graphics += "*";
                        is_empty = false;
                    }

                    snake.iter().for_each(|element| {
                        if element.position_x == x
                            && element.position_y == y
                            && element.position_x > 1
                            && element.position_y > 1
                            && element.position_x < CONSOLE_WIDTH - 1
                            && element.position_y < CONSOLE_HEIGHT - 1
                        {
                            graphics += "*";
                            is_empty = false;
                        }
                    });

                    if x == CONSOLE_WIDTH - 1 || x == 1 {
                        graphics += "|";
                        is_empty = false;
                    } else if y == CONSOLE_HEIGHT - 1 || y == 1  {
                        graphics += "_";
                        is_empty = false;
                    }

                    if is_empty {
                        graphics += " ";
                    }

                    x += 1;

                    if x == CONSOLE_WIDTH {
                        x = 0;
                        y += 1;
                    }
                    if y == CONSOLE_HEIGHT {
                        y = 0;
                    }

                    i += 1;
                }

                if snake[0].position_x == 1 {
                    snake[0].position_x = CONSOLE_WIDTH - 2;
                } else if snake[0].position_y == 1 {
                    snake[0].position_y = CONSOLE_HEIGHT - 2;
                } else if snake[0].position_x == CONSOLE_WIDTH - 1 {
                    snake[0].position_x = 2;
                } else if snake[0].position_y == CONSOLE_HEIGHT - 1 {
                    snake[0].position_y = 2;
                }

                let mut snake_piece_index = 3;

                while snake_piece_index < snake_len {
                    if snake[0].position_x == snake[snake_piece_index].position_x
                        && snake[0].position_y == snake[snake_piece_index].position_y
                    {
                        is_game_over = true;
                    }
                    snake_piece_index += 1;
                }

                if snake[0].position_x == food_position_x && snake[0].position_y == food_position_y
                {
                    spawn_food(&mut food_position_x, &mut food_position_y);

                    let snake_piece = SnakePiece::new();
                    snake.push(snake_piece);
                    snake_len = &snake.len() - 1;
                }

                clear_console();

                set_text_color(LIGHT_BLUE);
                println!("  Score {}", snake_len);

                let mut render_step = 0;

                while render_step < CONSOLE_HEIGHT {
                    let slice = &graphics[0..CONSOLE_WIDTH as usize];

                    set_text_color(YELLOW);
                    let stdout = io::stdout(); // get the global stdout entity
                    let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
                    writeln!(handle, "{}", slice); // add `?` if you care about errors here

                    graphics = String::from(&graphics[CONSOLE_WIDTH as usize..graphics.len()]);

                    render_step += 1;
                }
                println!("Control the snake w | a | s | d or the arrows. Pause spacebar or esc.");
            } else {
                clear_console();

                set_text_color(WHITE);
                let text = format!(
                    "Game over. Score {}. Press 'Enter' to start new game.",
                    snake.len()
                );

                text_center(text);

                update_render = false;
            }

            /*
            let stdout = io::stdout(); // get the global stdout entity
            let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
            writeln!(handle, "{}", graphics); // add `?` if you care about errors here
            */

            // вывожу результат замера производительности (сколько ушло времени на вывод текста в консоль)
            /*
            let stdout2 = io::stdout(); // get the global stdout entity
            let elapsed_time = now.elapsed();
            let mut handle2 = io::BufWriter::new(stdout2); // optional: wrap that handle in a buffer
            writeln!(handle2, "took {} nanoseconds.", elapsed_time.subsec_nanos()); // вывод сколько прошло времени
            println!("Running slow_function() took {} nanoseconds.", elapsed_time.subsec_nanos());
            */
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 10)); // контролируем количество обновлений за секунду
    }
}
