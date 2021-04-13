use bevy::prelude::*;
use rand;

/// количество ячеек (поле квадратное)
const FIELD_SIZE: i8 = 50;
/// размер окна в пикселях (окно квадратное)
const WINDOW_SIZE: f32 = 700.0;

/// размер одной ячейки
const CELL_SIZE: f32 = WINDOW_SIZE / FIELD_SIZE as f32;

fn main() {
    App::build()
        //настройка окна
        .insert_resource(WindowDescriptor
        {
            resizable: false,
            width: WINDOW_SIZE,
            height: WINDOW_SIZE,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        //регистрация ресурсов. ресурс один на приложение
        .init_resource::<Snake>()
        .init_resource::<Direction>()
        .init_resource::<Food>()
        .init_resource::<Ids>()
        // регистрация таймера, определеющего скорость игры
        .insert_resource(Tick(Timer::from_seconds(0.1, true)))
        .add_startup_system(setup.system())
        .add_system(keys_handle.system())
        .add_system(move_snake.system())
        .add_system(show_snake.system())
        .run()
}

/// определяет местоположение еды
#[derive(Debug)]
struct Food([i8; 2]);

impl Default for Food
{
    fn default() -> Self
    {
        Self([
            rand::random::<i8>().rem_euclid(FIELD_SIZE),
            rand::random::<i8>().rem_euclid(FIELD_SIZE)
        ])
    }
}

/// пользовательский ресурс для таймера (что-то вроде псевдонима, ну и избежание копий ресурса)
struct Tick(Timer);

/// координаты тела змейки. конец - голова. начало - хвост
#[derive(Debug)]
struct Snake(Vec<[i8; 2]>);

impl Default for Snake { fn default() -> Self { Self(vec![[0, 0], [0, 1], [0, 2]]) } }

fn setup(mut commands: Commands)
{
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}


///направление движения змейки
#[derive(Debug, Copy, Clone)]
enum  Direction
{
    Up, Down, Left, Right
}

impl Default for Direction { fn default() -> Self { Self::Up } }

/// система обработки пользовательского ввода
fn keys_handle(keys: Res<Input<KeyCode>>, mut direction: ResMut<Direction>)
{
    if keys.pressed(KeyCode::W)
    {
        match *direction {
            Direction::Down => return,
            _ => *direction = Direction::Up
        }
    }
    else if keys.pressed(KeyCode::S)
    {
        match *direction {
            Direction::Up => return,
            _ => *direction = Direction::Down
        }
    }
    else if keys.pressed(KeyCode::A)
    {
        match *direction {
            Direction::Right => return,
            _ => *direction = Direction::Left
        }
    }
    else if keys.pressed(KeyCode::D)
    {
        match *direction {
            Direction::Left => return,
            _ => *direction = Direction::Right
        }
    };
}

/// служебный ресурс для хранения созданных объектов (для возможности их удаления)
#[derive(Default)]
struct Ids(Vec<Entity>);


fn move_snake
(
    dir: Res<Direction>,
    mut snake: ResMut<Snake>,
    mut food: ResMut<Food>,
    mut tick: ResMut<Tick>,
    time: Res<Time>
)
{
    if tick.0.tick(time.delta()).just_finished()
    {
        let mut head = snake.0.last().unwrap().clone();
        match *dir
        {
            Direction::Up => head[1] = (head[1] + 1).rem_euclid(FIELD_SIZE),
            Direction::Down => head[1] = (head[1] - 1).rem_euclid(FIELD_SIZE),
            Direction::Left => head[0] = (head[0] - 1).rem_euclid(FIELD_SIZE),
            Direction::Right => head[0] = (head[0] + 1).rem_euclid(FIELD_SIZE),
        }
        if head != food.0 { snake.0.remove(0); } else
        {
            *food = Food::default();
        };
        if snake.0.contains(&head)
        {
            *snake = Snake::default()
        }
        else { snake.0.push(head) }
    }
}

fn show_snake(snake: Res<Snake>,
              food: Res<Food>,
              mut commands: Commands,
              mut windows: ResMut<Windows>,
              mut materials: ResMut<Assets<ColorMaterial>>,
              mut ids: ResMut<Ids>)
{
    for window in windows.iter_mut()
    {
        window.set_title(format!("Score: {}", snake.0.len()))
    }
    let material = materials.add(ColorMaterial::color(Color::rgb(0.0, 1.0, 1.0)));
    let sprite = Sprite::new(Vec2::new(CELL_SIZE, CELL_SIZE));
    while ids.0.len() != 0
    {
        let id = ids.0.pop().unwrap();
        commands.entity(id).despawn();
    }
    for cell in snake.0.iter()
    {
        let id = commands.spawn_bundle(SpriteBundle
        {
            transform: get_translation(cell),
            sprite: sprite.clone(),
            material: material.clone(),
            ..Default::default()
        }).id();
        ids.0.push(id)
    }
    ids.0.push(commands.spawn_bundle(SpriteBundle{
        transform: get_translation(&food.0),
        sprite, material: materials.add(ColorMaterial::color(Color::rgb(1.0, 0.5, 0.5))),
        ..Default::default()
    }).id());
}

/// получение реального местоположения из координат
fn get_translation(cell: &[i8; 2]) -> Transform
{
    let (row, column) = (cell[0], cell[1]);
    Transform::from_translation(
        Vec3::new((row - FIELD_SIZE / 2) as f32 * CELL_SIZE + CELL_SIZE * 0.5,
                  (column - FIELD_SIZE / 2) as f32 * CELL_SIZE + CELL_SIZE * 0.5, 0.0))
}