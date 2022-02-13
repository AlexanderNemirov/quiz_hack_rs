import subprocess as sb
import sys
import threading

def timeout():
    print('timeout')
    exit(3)

letters_arr = ["тетачасчастьчетачечет",
            "уходолгнефтьдымрубль",
            "выпускднопочватемпвода",
            "судстолканалполетштат",
            "мигзолотоначалостихлист",
            "данныелесгодцифраозеро",
            "имяродительврагшеяформула",
            #fat tests
            "экономикатеатрновоегорломинистр",
            "оборотмэрмоментдухсобака",
            #fattest test
            "методсонрисинженеркампания"]
            #program breaking test
            #"авгурвыжиглипомаотрывшрот"]

if __name__=='__main__':
    words='word_rus1000.txt'
    for opt in sys.argv:
        if opt=='-s':
            words='1000'
        if opt=='-m':
            words='word_rus34010.txt'
        if opt=='-l':
            words='word_rus51301.txt'
    t = threading.Timer(10, timeout)
    for letters in letters_arr:
        print("letters: ", letters)
        proc = sb.Popen('./target/debug/quiz_hack_rs data/'+words+' '+letters, shell=True,
                        stdout=sb.PIPE, stdin=sb.PIPE, stderr=sb.PIPE, 
                        universal_newlines=True, bufsize=1)
        while True:
            nextline = proc.stdout.readline()
            if nextline == '' and proc.poll() is not None:
                break
            sys.stdout.write(nextline)
            sys.stdout.flush()
        print()
    t.cancel()
