class vec3:
    def __init__(self, x: int, y: int, z: int) -> None:
        """Constructer for 3D Vector"""
        self.x = float(x)
        self.y = float(y)
        self.z = float(z)

    def negate(self):
        return vec3(-self.x, -self.y, -self.z)

    def add(self, other):
        return vec3(self.x + other.x, self.y + other.y, self.z + other.z)

    def sub(self, other):
        return vec3(self.x - other.x, self.y - other.y, self.z - other.z)

    def mul(self, other):
        return vec3(self.x * other.x, self.y * other.y, self.z * other.z)

    def div(self, other):
        return vec3(self.x / other.x, self.y / other.y, self.z / other.z)

    def dot(self, other):
        return vec3(self.x * other.x + self.y * other.y + self.z * other.z)

    def length(self):
        return self.dot(self).sqrt()
    
    def cross(self, other):
        return vec3(self.y * other.z - self.z * other.y,
                    self.z * other.x - self.x * other.z,
                    self.x * other.y - self.y * other.x)

    def normalized(self):
        return self / self.length()