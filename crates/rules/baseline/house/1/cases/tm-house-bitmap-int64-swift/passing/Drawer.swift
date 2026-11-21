public struct Drawer {
    public static let flagArchived: Int64 = 1 << 3
    public var id: Int64
    public var title: String
    public var operationalBitmap: Int64 = 0
    public var isArchived: Bool {
        get { operationalBitmap & Drawer.flagArchived != 0 }
        set { operationalBitmap = newValue ? operationalBitmap | Drawer.flagArchived : operationalBitmap & ~Drawer.flagArchived }
    }
}
