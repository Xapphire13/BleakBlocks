//
//  Block.swift
//  BlockBreaker
//

import SpriteKit

class Block: SKSpriteNode {
    var coordinate: CGPoint = CGPoint(x: 0, y: 0)
    
    convenience init(color: SKColor, size: CGSize, coordinate: CGPoint) {
        self.init(texture: nil, color: color, size: size)
        self.coordinate = coordinate
    }
}
