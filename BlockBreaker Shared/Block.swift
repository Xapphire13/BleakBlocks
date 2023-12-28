//
//  Block.swift
//  BlockBreaker
//

import SpriteKit

class Block: SKSpriteNode {
    var isHighlighted = false
    var originalColor: SKColor?
    
    convenience init(color: SKColor, size: CGSize) {
        self.init(texture: nil, color: color, size: size)
        self.originalColor = color
    }
    
    func highlight() {
        if (self.isHighlighted) {
            return
        }
        
        self.isHighlighted = true
        
        let highlightAction = SKAction.colorize(with: .white, colorBlendFactor: 0.5, duration: 0.2)
        run(highlightAction)
    }

    func unhighlight() {
        self.isHighlighted = false
        
        if let originalColor = self.originalColor {
            let unhighlightAction = SKAction.colorize(with: originalColor, colorBlendFactor: 1.0, duration: 0.2)
            run(unhighlightAction)
        }
    }
}
