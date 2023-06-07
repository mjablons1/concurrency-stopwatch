import java.util.Scanner;

public class Stopwatch {

    class Control {
        public final int refreshRateMsec = 1000;
        public final int counterRateMsec = 100;

        public final int countsPerSec = 1000 / counterRateMsec;
        public final int countsPerMin = 60 * countsPerSec;
        public final int countsPerHour = 3600 * countsPerSec;
        public volatile int count = 0;
        public volatile boolean run = true;
      }

    final Control control = new Control();

    class Counter
        implements Runnable {
        public void run() { 
            try {
                while(control.run){
                    Thread.sleep(control.counterRateMsec);
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
                    System.out.format(countToTime(control.count));
                    Thread.sleep(control.refreshRateMsec);
                }
            } catch (InterruptedException e) {
                System.out.format("Refresh thread was interrupted");
            }
        }
    }
    
    public String countToTime(int count){
        int hours = count / control.countsPerHour;
        int reminder = count % control.countsPerHour;
        int minutes = reminder / control.countsPerMin;
        int reminder2 = reminder % control.countsPerMin;
        int seconds = reminder2 / control.countsPerSec;
        int milisec = reminder2 % control.countsPerSec;
        
        return (hours + ":" + minutes + ":" + seconds + ":" + milisec + "\n");
    }

    public void readInput(){
        Scanner scanner = new Scanner(System.in);

        while (true){
            String keyInput = scanner.nextLine();
            if (keyInput.equals("q")){
                control.run = false;
                break;
            }
            else if (keyInput.equals("r")){
                control.count = 0;
            }
        }
        scanner.close();
    }

    public static void main(String[] args)
    throws InterruptedException {
        Stopwatch stopwatch = new Stopwatch();

        Thread counterThread = new Thread(stopwatch.new Counter());
        Thread refreshThread = new Thread(stopwatch.new Refresh());

        refreshThread.start();
        counterThread.start();

        stopwatch.readInput();

        refreshThread.join();
        counterThread.join();
    }
        
}
