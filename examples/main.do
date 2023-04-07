
Player {
    name: String
    currentAnimation: String
    children: () => {
        return [
            RigidBody {
                shape: Box {
                    width: 100
                    height: 100
                    depth: 100
                }
                onCollision: () => {

                }
            }
            Asset {
                path: "person.glb"

            }
        ]
    }
}

let players = [
    Player {
        name: "matti"
    },
    Player {
        name: "teppo"
    }
]



Main {
    children: [
        GameWorld {
            children: players.map(p => p) 
        }
    ]
}