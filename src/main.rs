use std::thread;
use std::io::{self, BufRead};
use std::fs::{File, OpenOptions};
use std::vec::Vec;
use workctl::WorkQueue;
use std::io::prelude::*;
use std::sync::{Mutex, Arc};

const N_THREADS: usize = 8;

const HASHES: [u64; 82] = [
    10374841591685794123,
    10484659978517092504,
    10501212300031893463,
    10734127004244879770,
    11073283311104541690,
    1109067043404435916,
    11109294216876344399,
    11266044540366291518,
    11385275378891906608,
    12574535824074203265,
    12785322942775634499,
    13029357933491444455,
    13135068273077306806,
    13464308873961738403,
    13599785766252827703,
    13655261125244647696,
    1367627386496056834,
    13693525876560827283,
    13783346438774742614,
    13799353263187722717,
    13852439084267373191,
    14079676299181301772,
    14243671177281069512,
    14513577387099045298,
    1475579823244607677,
    15267980678929160412,
    15514036435533858158,
    15535773470978271326,
    155978580751494388,
    16066651430762394116,
    16112751343173365533,
    16130138450758310172,
    1682585410644922036,
    16858955978146406642,
    16990567851129491937,
    17291806236368054941,
    17439059603042731363,
    17624147599670377042,
    17683972236092287897,
    17849680105131524334,
    17956969551821596225,
    17997967489723066537,
    18246404330670877335,
    18446744073709551613,
    191060519014405309,
    2380224015317016190,
    2589926981877829912,
    3320767229281015341,
    3425260965299690882,
    3642525650883269872,
    3769837838875367802,
    3778500091710709090,
    3869935012404164040,
    4030236413975199654,
    4088976323439621041,
    4578480846255629462,
    506634811745884560,
    5132256620104998637,
    5183687599225757871,
    541172992193764396,
    5415426428750045503,
    5449730069165757263,
    6116246686670134098,
    6180361713414290679,
    640589622539783622,
    6461429591783621719,
    6508141243778577344,
    6827032273910657891,
    700598796416086955,
    7574774749059321801,
    7701683279824397773,
    7775177810774851294,
    7878537243757499832,
    8381292265993977266,
    8408095252303317471,
    8612208440357175863,
    8727477769544302060,
    8994091295115840290,
    9007106680104765185,
    9234894663364701749,
    9555688264681862794,
    9903758755917170407
];

// Private Shared Function GetHash(s As String) As ULong
//      Dim num As ULong = 14695981039346656037UL
//      Try
//              For Each b As Byte In Encoding.UTF8.GetBytes(s)
//                      num = num Xor CULng(b)
//                      num *= 1099511628211UL
//              Next
//      Catch
//      End Try
//      Return num Xor 6605813339339102567UL
// End Function

/// Calculate the silly hash
#[inline]
fn calc_hash(input: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for b in input {
        hash ^= *b as u64;
        hash = 0x100_0000_01b3u64.wrapping_mul(hash);
    }

    hash ^ 0x5bac_903b_a7d8_1967u64
}

fn await_and_write(handle: Arc<Mutex<File>>, strname: &str, hash: u64) {
    let mut fs = handle.lock().unwrap();
    writeln!(fs, "{}:{}", strname, hash).unwrap();
}

fn main() -> io::Result<()> {

    let mut work_queue: WorkQueue<std::string::String> = WorkQueue::new();

    let thread_ids: Vec<_> = (0..N_THREADS).collect();

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("found.txt")
        .unwrap();

    let file_mutex = Arc::new(Mutex::new(file));

    let mut tid = 0;

    let threads: Vec<_> = thread_ids.into_iter().map(|_| {
        let mut thread_wq = work_queue.clone();
        let file_handle = file_mutex.clone();
        let mut n: usize = 0;
        tid += 1;

        thread::spawn(move || {
            loop {
                if n % 10_000_000 == 0 {
                    println!("TID [{}]: {}", tid, n);
                }

                if let Some(procname) = thread_wq.pull_work() {
                    let procname = procname.into_bytes();
                    let hash = calc_hash(&procname);
                    if HASHES.contains(&hash) {
                        let file_handle = file_handle.clone();
                        let strname = std::str::from_utf8(&procname).unwrap();

                        println!("[!] {}:{}",
                            strname,
                            hash);

                        await_and_write(file_handle, strname, hash);
                    }
                } else {
                    thread::yield_now();
                }
                n += 1;
            }
        })
    }).collect();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        work_queue.push_work(line?);
    }

    for handle in threads {
        handle.join().unwrap();
    }

    Ok(())
}
