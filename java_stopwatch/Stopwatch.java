import java.util.Scanner;

public class Stopwatch {

    class Control {
        public final int refresh_rate_msec = 1000;
        public final int counter_rate_msec = 100;

        public final int counts_per_sec = 1000 / counter_rate_msec;
        public final int counts_per_min = 60 * counts_per_sec;
        public final int counts_per_hour = 3600 * counts_per_sec;
        public volatile int count = 0;
        public volatile boolean run = true;
      }

    final Control control = new Control();

    class Counter
        implements Runnable {
        public void run() { 
            try {
                while(control.run){
                    Thread.sleep(control.counter_rate_msec);
                    control.count+=1; // to be safe we might want to increase count from a separate, "synchronized void increment()" method, but same issue as in python...
                }
            } catch (InterruptedException e) {
                System.out.format("Counter thread was interrupted!");
            }
        }
    }

    class Refresh
        implements Runnable {
        public void run() {
            try {
                while(control.run){
                    //System.out.print("\033[H\033[2J");
                    //System.out.flush();
                    System.out.format(count_to_time(control.count));
                    Thread.sleep(control.refresh_rate_msec);
                }
            } catch (InterruptedException e) {
                System.out.format("Refresh thread was interrupted");
            }
        }
    }
    
    public String count_to_time(int count){
        int hours = count / control.counts_per_hour;
        int reminder = count % control.counts_per_hour;
        int minutes = reminder / control.counts_per_min;
        int reminder2 = reminder % control.counts_per_min;
        int seconds = reminder2 / control.counts_per_sec;
        int milisec = reminder2 % control.counts_per_sec;
        
        return (hours + ":" + minutes + ":" + seconds + ":" + milisec + "\n");
    }

    public void read_input(){
        Scanner scanner = new Scanner(System.in);

        while (true){
            String key_input = scanner.nextLine();
            if (key_input.equals("q")){
                control.run = false;
                break;
            }
            else if (key_input.equals("r")){
                control.count = 0;
            }
        }
        scanner.close();
    }

    public static void main(String[] args)
    throws InterruptedException {
        Stopwatch stopwatch = new Stopwatch();

        Thread counter_thread = new Thread(stopwatch.new Counter());
        Thread refresh_thread = new Thread(stopwatch.new Refresh());

        refresh_thread.start();
        counter_thread.start();

        stopwatch.read_input();

        refresh_thread.join();
        counter_thread.join();
    }
        
}
