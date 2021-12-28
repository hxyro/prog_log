print(os.time())
print(os.date())
print(os.getenv("PWD"))
io.output("./kkk/abc")
io.output("ehh")
io.close()
os.rename("abc","qwert")
os.remove("ehh")
os.execute("ls | lolcat")

for i=1,10 do
--    print(i)
    if i == 4 then
        os.exit()
    end
end

