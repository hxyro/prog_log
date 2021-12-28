table = {
    name = "hmm",
    age = "3232",
    job = "none",
}
function pet(name)
    name = name or "misk"
    return {
        name = name,
        status = "hungry",
        alive = true,
        feed = function(self)
                    self.status = "full"
                end
    }
end    

print(table.name)

local cat = pet("meow")
if cat.alive then
print(cat.name)
end
dog = pet()
if dog.alive then
print(dog.name)
end

print(dog.status)
dog:feed()
print(dog.status)

local function doge(name, breed)
    local doge = pet(name)
    doge.breed = breed
    return doge
end
local y = doge("doge", "shiba")
print(y.name)
print(y.breed)


