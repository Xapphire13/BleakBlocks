//
//  GameScene.swift
//  BlockBreaker Shared
//

import SpriteKit

class GameScene: SKScene {
    var blocks: [[Block]] = []
    let blockSize = CGSize(width: 50, height: 50)

    class func newGameScene() -> GameScene {
        // Load 'GameScene.sks' as an SKScene.
        guard let scene = SKScene(fileNamed: "GameScene") as? GameScene else {
            print("Failed to load GameScene.sks")
            abort()
        }
        
        // Set the scale mode to scale to fit the window
        scene.scaleMode = .aspectFill
        
        return scene
    }

    override func didMove(to view: SKView) {
        self.setupGame()
    }

    func setupGame() {
        for row in 0..<5 {
            var rowBlocks: [Block] = []

            for col in 0..<5 {
                let block = Block(
                    color: SKColor.magenta,
                    size: blockSize,
                    coordinate: CGPoint(x: col, y: row)
                )
                block.position = CGPoint(x: col + col * Int(blockSize.width), y: row + row * Int(blockSize.height))
                block.name = "block\(row)_\(col)"
                addChild(block)

                rowBlocks.append(block)
            }

            blocks.append(rowBlocks)
        }
    }
    
    func removeBlock(_ block: Block) {
        // Animate removal
        let fadeOutAction = SKAction.fadeOut(withDuration: 0.5)
        let removeAction = SKAction.removeFromParent()
        block.run(SKAction.sequence([fadeOutAction, removeAction]))

        // Update game state (remove block from the array)
        if let index = blocks.firstIndex(where: { $0.contains(block) }) {
            blocks[index].removeAll { $0 == block }
        }
    }


    func selectAndRemoveBlocks(startingFrom block: Block) {
        self.removeBlock(block)
    }
    
    func removeAndShiftBlocks(selectedBlocks: [Block]) {
        // Implement block removal and column shifting logic
    }
    
}

#if os(iOS) || os(tvOS)
// Touch-based event handling
extension GameScene {
    override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) {
        touches.forEach { touch in
            let location = touch.location(in: self)
            let node = atPoint(location)

            if let block = node as? Block {
                // Handle block selection and removal
                self.selectAndRemoveBlocks(startingFrom: block)
            }
        }
    }
}
#endif

#if os(OSX)
// Mouse-based event handling
extension GameScene {
    override func mouseUp(with event: NSEvent) {
        let location = event.location(in: self)
        let node = atPoint(location)

        if let block = node as? Block {
            // Handle block selection and removal
            self.selectAndRemoveBlocks(startingFrom: block)
        }
    }
}
#endif

