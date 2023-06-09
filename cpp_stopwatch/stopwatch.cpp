#include <thread>
#include <chrono>
#include <iostream>

using namespace std;

class Stopwatch{
    public:

        Stopwatch(){
            counts_per_sec = 1000/counter_rate_msec;
            counts_per_min = 60 * counts_per_sec;
            counts_per_hour = 3600 * counts_per_sec;  
        };

        void counter(){
            while(is_running){
                this_thread::sleep_for(chrono::milliseconds(counter_rate_msec));
                count++;
            }
        };

        void refresh(){
            while(is_running){
                cout << count_to_time() << endl;
                this_thread::sleep_for(chrono::milliseconds(refresh_rate_msec));
            }
        };

        string count_to_time(){
            string hours = to_string(count / counts_per_hour);
            int reminder = count % counts_per_hour;
            string minutes = to_string(reminder / counts_per_min);
            int reminder2 = reminder % counts_per_min;
            string seconds = to_string(reminder2 / counts_per_sec);
            string milisec = to_string(reminder2 % counts_per_sec);
            return hours + ":" + minutes + ":" + seconds + ":" + milisec;
        };

        void read_input(){
            char resp;
            while(true){
                cin >> resp;
                if(resp == 'q') {
                    is_running = false;
                    break;}
                else if(resp == 'r') {count = 0;}
            };
        };

    private:
        int refresh_rate_msec{1000};
        int counter_rate_msec{100};
        int counts_per_sec;
        int counts_per_min;
        int counts_per_hour;
        int count{0};
        bool is_running{true};
};

int main(){
    Stopwatch stopwatch;

    thread refresh_thread(&Stopwatch::refresh, &stopwatch);
    thread counter_thread(&Stopwatch::counter, &stopwatch);
    
    stopwatch.read_input();
    
    refresh_thread.join();
    counter_thread.join();
}