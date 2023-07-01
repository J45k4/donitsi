import Triangle

game_world = GameWorld {
    gravity: 50
    items: [
        Triangle {
            top: [0, 2]
            left: [-2, 0]
            right: [2, 0]
        }
    ]
}

Main {
    children: Window {
        width: 500
        height: 500
        title: "Tumma poika"
        children: [
            Camera {
                world: game_world
                location: [0,0,0]
                looking_at: [1,1,0]
            }
        ]
    }  
}
